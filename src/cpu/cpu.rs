use std::sync::Arc;
use parking_lot::Mutex;
use crate::cpu::game_boy_cpu::GameBoyCPU;
use crate::cpu::instructions::decode_instruction;
use crate::memory::MemoryController;

pub trait CPU {

    fn clock (&mut self, memory: Arc<Mutex<MemoryController>>);
}