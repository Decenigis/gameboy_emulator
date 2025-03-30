use std::cell::RefCell;
use std::ops::Add;
use std::rc::Rc;
use std::sync::Arc;
use parking_lot::Mutex;
use crate::cpu::CPU;
use crate::memory::MemoryController;



pub struct NullableCPU {
    pub num_times_clocked: Rc<RefCell<u32>>
}

impl CPU for NullableCPU {
    fn clock(&mut self, _memory: Arc<Mutex<MemoryController>>) {
        let result = self.num_times_clocked.borrow().add(1);
        self.num_times_clocked.replace(result);
    }
}

impl NullableCPU {
    pub fn new(num_times_clocked: Rc<RefCell<u32>>) -> Self {
        Self {
            num_times_clocked
        }
    }
}
