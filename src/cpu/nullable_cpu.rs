#![allow(dead_code)]

use std::cell::RefCell;
use std::ops::Add;
use std::rc::Rc;
use std::sync::Arc;
use parking_lot::Mutex;
use crate::cpu::CPU;
use crate::cpu::interrupt::Interrupt;
use crate::memory::MemoryController;



pub struct NullableCPU {
    pub num_times_clocked: Rc<RefCell<u32>>,
    pub interrupt_requested: Rc<RefCell<Option<Interrupt>>>,
}

impl CPU for NullableCPU {

    fn clock(&mut self, _memory: Arc<Mutex<MemoryController>>) {
        let result = self.num_times_clocked.borrow().add(1);
        self.num_times_clocked.replace(result);
    }

    fn try_interrupt(&mut self, _memory: Arc<Mutex<MemoryController>>, interrupt: Interrupt) {
        self.interrupt_requested.replace(Some(interrupt));
    }
}

impl NullableCPU {
    pub fn new(num_times_clocked: Rc<RefCell<u32>>, interrupt_requested: Rc<RefCell<Option<Interrupt>>>) -> Self {
        Self {
            num_times_clocked,
            interrupt_requested
        }
    }
}
