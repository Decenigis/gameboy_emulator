use std::sync::Arc;
use parking_lot::Mutex;
use crate::cpu::alu::ALU;
use crate::cpu::instructions::Instruction;
use crate::cpu::registers::Registers;
use crate::memory::MemoryController;

pub struct Ei {}

impl Instruction for Ei {

    #[inline]
    fn from_opcode(opcode: &u8) -> Option<Box<dyn Instruction>> {
        if *opcode == 0xFB {
            return Some(Box::new(Ei {}))
        }
        None
    }

    fn get_opcode(&self) -> u8 {
        0xFB
    }

    fn act(&mut self, _registers: &mut Registers, _alu: &mut ALU, _memory_controller: Arc<Mutex<MemoryController>>, enable_interrupts: &mut bool) -> bool {
        *enable_interrupts = true;

        true
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    reusable_testing_macro!(0xFB, Ei);

    #[test]
    fn act_immediately_returns_true() {
        let mut registers = Registers::new(0, 0, 0, 0, 0, 0);
        let mut alu = ALU::new(registers.f.clone());

        let mut instruction = Ei {};

        let result = instruction.act(&mut registers, &mut alu, Arc::new(Mutex::new(MemoryController::new())) ,&mut false);

        assert_eq!(true, result);
    }

    #[test]
    fn act_enables_interrupts() {
        let mut registers = Registers::new(0, 0, 0, 0, 0, 0);
        let mut alu = ALU::new(registers.f.clone());

        let mut instruction = Ei {};
        let mut enable_interrupts = false;

        instruction.act(&mut registers, &mut alu, Arc::new(Mutex::new(MemoryController::new())), &mut enable_interrupts);

        assert_eq!(true, enable_interrupts);
    }
}
