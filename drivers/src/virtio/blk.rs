use lock::Mutex;
use virtio_drivers::{VirtIOBlk as InnerDriver, VirtIOHeader};
use virtio_drivers::{VirtIOBlkPCI as InnerDriverPCI, VirtIOPCIHeader};

use crate::scheme::{BlockScheme, Scheme};
use crate::DeviceResult;

pub struct VirtIoBlk<'a> {
    inner: Mutex<InnerDriver<'a>>,
}

impl<'a> VirtIoBlk<'a> {
    pub fn new(header: &'static mut VirtIOHeader) -> DeviceResult<Self> {
        Ok(Self {
            inner: Mutex::new(InnerDriver::new(header)?),
        })
    }
}

impl<'a> Scheme for VirtIoBlk<'a> {
    fn name(&self) -> &str {
        "virtio-blk"
    }

    fn handle_irq(&self, _irq_num: usize) {
        self.inner.lock().ack_interrupt();
    }
}

impl<'a> BlockScheme for VirtIoBlk<'a> {
    fn read_block(&self, block_id: usize, buf: &mut [u8]) -> DeviceResult {
        self.inner.lock().read_block(block_id, buf)?;
        Ok(())
    }

    fn write_block(&self, block_id: usize, buf: &[u8]) -> DeviceResult {
        self.inner.lock().write_block(block_id, buf)?;
        Ok(())
    }

    fn flush(&self) -> DeviceResult {
        Ok(())
    }
}

pub struct VirtIoBlkPCI<'a> {
    inner: Mutex<InnerDriverPCI<'a>>,
}

impl<'a> VirtIoBlkPCI<'a> {
    pub fn new(header: VirtIOPCIHeader) -> DeviceResult<Self> {
        Ok(Self {
            inner: Mutex::new(InnerDriverPCI::new(header)?),
        })
    }
}

impl<'a> Scheme for VirtIoBlkPCI<'a> {
    fn name(&self) -> &str {
        "virtio-blk-pci"
    }

    fn handle_irq(&self, _irq_num: usize) {
        self.inner.lock().ack_interrupt();
    }
}

impl<'a> BlockScheme for VirtIoBlkPCI<'a> {
    fn read_block(&self, block_id: usize, buf: &mut [u8]) -> DeviceResult {
        self.inner.lock().read_block(block_id, buf)?;
        Ok(())
    }

    fn write_block(&self, block_id: usize, buf: &[u8]) -> DeviceResult {
        self.inner.lock().write_block(block_id, buf)?;
        Ok(())
    }

    fn flush(&self) -> DeviceResult {
        Ok(())
    }
}