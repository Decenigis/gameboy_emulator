use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;
use parking_lot::Mutex;
use crate::cpu::CPU;
use crate::memory::MemoryController;

pub struct NullableCPUInternal {
    pub num_times_clocked: u32
}


pub struct NullableCPU {
    internal: Rc<RefCell<NullableCPUInternal>>
}

impl CPU for NullableCPU {
    fn clock(&mut self, _memory: Arc<Mutex<MemoryController>>) {
        self.internal.borrow_mut().num_times_clocked += 1
    }
}

impl NullableCPU {
    pub fn new(internal: Rc<RefCell<NullableCPUInternal>>) -> Self {
        Self {
            internal
        }
    }
}