use core::future::Future;
use core::task::{Poll, Waker, RawWaker, RawWakerVTable, Context};
use core::pin::Pin;
use alloc::sync::Arc;
use alloc::boxed::Box;
use alloc::collections::VecDeque;
use super::Thread;
use crate::{ThreadFn, CurrentThread, Syscall};
use spin::Mutex;
use lazy_static::*;

/// Polled by the application core, wrap the behavior of the async syscall.
pub struct RemoteSyscallFuture {
    current_thread: Arc<Thread>,
    thread_fn: ThreadFn,
    num: u32,
    args: [usize; 6],
    completed: bool,
    ret: isize,
}

impl Future for RemoteSyscallFuture {
    type Output = isize;
    fn poll(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Self::Output> {
        if !self.completed {
            let waker = cx.waker().clone();
            let req = RemoteSyscallRequest {
                current_thread: self.current_thread.clone(),
                thread_fn: self.thread_fn,
                num: self.num,
                args: self.args,
                completed_ptr: &self.completed as *const _ as usize,
                ret_ptr: &self.ret as *const _ as usize,
                wakeup_fn: Box::new(move || { waker.wake_by_ref(); }),
            };
            REMOTE_SYSCALL_REQUESTS.lock().push_front(req);
            Poll::Pending
        } else {
            Poll::Ready(self.ret)
        }
    }
}

/// Called by the application core. Issue an async syscall request to the second core.
pub fn remote_syscall(current_thread: Arc<Thread>, thread_fn: ThreadFn, num: u32, args: [usize; 6],) -> Pin<Box<RemoteSyscallFuture>> {
    Box::pin(RemoteSyscallFuture {
        current_thread,
        thread_fn,
        num,
        args,
        completed: false,
        ret: 0,
    })
}

/// Contains all information about the syscall execution on the second core.
pub struct RemoteSyscallRequest {
    pub current_thread: Arc<Thread>,
    thread_fn: ThreadFn,
    num: u32,
    args: [usize; 6],
    completed_ptr: usize,
    ret_ptr: usize,
    wakeup_fn: Box<dyn FnOnce() -> () + Send + 'static>,
}

lazy_static! {
    static ref REMOTE_SYSCALL_REQUESTS: Mutex<VecDeque<RemoteSyscallRequest>> = Mutex::new(VecDeque::new());
}

/// An empty waker.
/// Usage:
/// ```rust
/// let raw_waker = RawWaker::new(core::ptr::null(), &EMPTY_WAKER_TABLE);
/// let waker = unsafe { Waker::from_raw(raw_waker) };
/// let mut cx = Context::from_waker(&waker);
/// ```
const EMPTY_WAKER_TABLE: RawWakerVTable = RawWakerVTable::new(
    |data| RawWaker::new(data, &EMPTY_WAKER_TABLE),
    |_data| {},
    |_data| {},
    |_data| {},
);

/// Main event loop of async syscall module.
#[allow(unsafe_code)]
pub fn event_loop() -> ! {
    warn!("Hello world!");
    loop {
        if let Some(req) = REMOTE_SYSCALL_REQUESTS.lock().pop_back() {
            let raw_waker = RawWaker::new(core::ptr::null(), &EMPTY_WAKER_TABLE);
            let waker = unsafe { Waker::from_raw(raw_waker) };
            let mut cx = Context::from_waker(&waker);


            /* original code was:
            let mut syscall = linux_syscall::Syscall {
                thread,
                thread_fn,
                syscall_entry: kernel_hal::context::syscall_entry as usize,
            };
            trace!("Syscall : {} {:x?}", num as u32, args);
            run_with_irq_enable! {
                let ret = syscall.syscall(num as u32, args).await as usize
            }
            */

            let current_thread = CurrentThread(req.current_thread.clone());
            let mut syscall = Syscall {
                thread: &current_thread,
                thread_fn: req.thread_fn,
                syscall_entry: kernel_hal::context::syscall_entry as usize,
            };
            use cfg_if::cfg_if;
            cfg_if! {
                if #[cfg(all(target_os = "none", target_arch = "aarch64"))] {
                    use kernel_hal::arch::config::USER_TABLE_FLAG;
                    kernel_hal::vm::activate_paging(req.current_thread.proc().vmar().table_phys() | USER_TABLE_FLAG);
                } else {
                    kernel_hal::vm::activate_paging(req.current_thread.proc().vmar().table_phys());
                }
            }
            let mut future = Box::pin(syscall.syscall(req.num as u32, req.args));
            if let Poll::Ready(ret) = future.as_mut().poll(&mut cx) {
                unsafe {
                    (req.completed_ptr as *mut bool).write_volatile(true);
                    (req.ret_ptr as *mut isize).write_volatile(ret);
                }
                (req.wakeup_fn)();
            } else {
                error!("the system call num {} does not return immediately", req.num);
                loop {}
            }
        }
    }
}