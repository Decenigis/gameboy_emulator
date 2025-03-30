use std::sync::Arc;
use parking_lot::Mutex;
use crate::memory::MemoryController;

pub trait CPU {

    fn clock (&mut self, memory: Arc<Mutex<MemoryController>>);
}
