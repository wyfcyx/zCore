use core::future::Future;
use core::task::{Poll, Context};
use core::pin::Pin;
use core::sync::atomic::{AtomicUsize, AtomicBool, Ordering};
use alloc::sync::Arc;
use alloc::boxed::Box;
use alloc::collections::{VecDeque, BTreeMap};
use super::Thread;
use crate::{ThreadFn, CurrentThread, Syscall};
use spin::Mutex;
use lazy_static::*;
use woke::Woke;

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
            REMOTE_SYSCALL_REQUESTS.push_back(req);
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
    wakeup_fn: Box<dyn FnOnce() -> () + Send + Sync + 'static>,
}

pub struct RemoteSyscallSubmissionQueue {
    pub empty: AtomicBool,
    pub queue: Mutex<VecDeque<RemoteSyscallRequest>>,
}

impl RemoteSyscallSubmissionQueue {
    pub fn new() -> Self {
        Self {
            empty: AtomicBool::new(true),
            queue: Mutex::new(VecDeque::new()),
        }
    }
    pub fn push_back(&self, req: RemoteSyscallRequest) {
        self.queue.lock().push_back(req);
        self.empty.store(false, Ordering::SeqCst);
    }
    pub fn pop_front(&self) -> Option<RemoteSyscallRequest> {
        if self.empty.load(Ordering::SeqCst) == false {
            let mut queue = self.queue.lock();
            let ret = queue.pop_front();
            if queue.is_empty() {
                self.empty.store(true, Ordering::SeqCst);
            }
            ret
        } else {
            None
        }
    }
}


lazy_static! {
    //static ref REMOTE_SYSCALL_REQUESTS: Mutex<VecDeque<RemoteSyscallRequest>> = Mutex::new(VecDeque::new());
    static ref REMOTE_SYSCALL_REQUESTS: RemoteSyscallSubmissionQueue = RemoteSyscallSubmissionQueue::new();
    static ref ASYNC_SYSCALL_MODULE: Arc<Mutex<AsyncSyscallModule>> = Arc::new(Mutex::new(AsyncSyscallModule::new()));
}

static ASYNC_TASK_ID: AtomicUsize = AtomicUsize::new(0);

type SyscallFuture = Pin<Box<dyn Future<Output = isize> + Send>>;

pub struct AsyncSyscallTask {
    pub id: usize,
    pub future: Arc<Mutex<SyscallFuture>>,
    pub req: RemoteSyscallRequest,
}

pub struct AsyncSyscallModule {
    pub task_map: BTreeMap<usize, AsyncSyscallTask>,
    pub ready_tasks: VecDeque<usize>,
    //pub pending_tasks: VecDeque<usize>,
}

impl AsyncSyscallModule {
    pub fn new() -> Self {
        Self {
            task_map: BTreeMap::new(),
            ready_tasks: VecDeque::new(),
            //pending_tasks: VecDeque::new(),
        }
    }
    pub fn add_task(&mut self, future: SyscallFuture, req: RemoteSyscallRequest) {
        ASYNC_TASK_ID.fetch_add(1, Ordering::SeqCst);
        let task_id = ASYNC_TASK_ID.load(Ordering::SeqCst);
        self.task_map.insert(task_id, AsyncSyscallTask {
            id: task_id,
            future: Arc::new(Mutex::new(future)),
            req,
        });
        self.ready_tasks.push_back(task_id);
    }
    pub fn wakeup_task(&mut self, task_id: usize) {
        // avoid the case that a completed task is woken up
        if self.task_map.get(&task_id).is_some() {
            self.ready_tasks.push_back(task_id);
        }
    }
    pub fn remove_task(&mut self, task_id: usize) -> AsyncSyscallTask {
        //info!("remove_task id = {}", task_id);
        self.task_map.remove(&task_id).expect("remove task failed")
    }
    pub fn select_task(&mut self) -> Option<usize> {
        self.ready_tasks.pop_front()
    }
}

struct AsyncSyscallWaker {
    pub module: Arc<Mutex<AsyncSyscallModule>>,
    pub task_id: usize,
}

impl Woke for AsyncSyscallWaker {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        let mut module = arc_self.module.lock();
        module.wakeup_task(arc_self.task_id);
    }
}

struct SyscallWithoutRef {
    pub thread: CurrentThread,
    pub thread_fn: ThreadFn,
    pub syscall_entry: usize,
}

/// Main event loop of async syscall module.
#[allow(unsafe_code)]
pub fn event_loop() -> ! {
    warn!("Hello world!");
    loop {
        let mut module = ASYNC_SYSCALL_MODULE.lock();
        // add new requests
        {
            while let Some(req) = REMOTE_SYSCALL_REQUESTS.pop_front() {
                info!("queue op completed, before adding new req");
                let syscall_without_ref = SyscallWithoutRef {
                    thread: CurrentThread(req.current_thread.clone()),
                    thread_fn: req.thread_fn,
                    syscall_entry: kernel_hal::context::syscall_entry as usize,
                };
                let num_copy = req.num;
                let args_copy = req.args;
                let future = Box::pin(async move {
                    let mut syscall = Syscall {
                        thread: &syscall_without_ref.thread,
                        thread_fn: syscall_without_ref.thread_fn,
                        syscall_entry: syscall_without_ref.syscall_entry,
                    };
                    syscall.syscall(num_copy, args_copy).await
                });
                module.add_task(future, req);
                info!("after adding new req");
            }
        }
        // we do not need to wakeup tasks, since they are woken up elsewhere
        
        // handle requests
        if let Some(task_id) = module.select_task() {
            info!("select task id: {}", task_id);
            let task = module.task_map.get(&task_id).unwrap();
            
            // generate waker
            let waker = Arc::new(AsyncSyscallWaker {
                module: ASYNC_SYSCALL_MODULE.clone(),
                task_id,
            });
            let waker = woke::waker_ref(&waker);
            let mut cx = Context::from_waker(&waker);
            // switch pagetable before polling
            use cfg_if::cfg_if;
            cfg_if! {
                if #[cfg(all(target_os = "none", target_arch = "aarch64"))] {
                    use kernel_hal::arch::config::USER_TABLE_FLAG;
                    kernel_hal::vm::activate_paging(task.req.current_thread.proc().vmar().table_phys() | USER_TABLE_FLAG);
                } else {
                    kernel_hal::vm::activate_paging(task.req.current_thread.proc().vmar().table_phys());
                }
            }
            // fetch future to poll
            let future = Arc::clone(&task.future);
            drop(module);
            // poll
            let mut future = future.lock();
            if let Poll::Ready(ret) = future.as_mut().poll(&mut cx) {
                let mut module = ASYNC_SYSCALL_MODULE.lock();
                let req = module.remove_task(task_id).req;
                info!("task {} dropped", task_id);
                unsafe {
                    (req.completed_ptr as *mut bool).write_volatile(true);
                    (req.ret_ptr as *mut isize).write_volatile(ret);
                }
                (req.wakeup_fn)();
                info!("wakeup client!");
            } else {
                info!("task {} pending", task_id);
            }
        }
    }
}