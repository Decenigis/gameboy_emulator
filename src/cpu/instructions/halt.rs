use crate::cpu::alu::ALU;
use crate::cpu::instructions::Instruction;
use crate::cpu::registers::Registers;
use crate::memory::MemoryController;
use parking_lot::Mutex;
use std::sync::Arc;

pub struct Halt {}

impl Instruction for Halt {

    #[inline]
    fn from_opcode(opcode: &u8) -> Option<Box<dyn Instruction>> {
        if *opcode == 0x76 {
            return Some(Box::new(Halt {}))
        }
        None
    }

    fn get_opcode(&self) -> u8 {
        0x76
    }

    fn act(&mut self, _registers: &mut Registers, _alu: &mut ALU, _memory_controller: Arc<Mutex<MemoryController>>, _enable_interrupts: &mut bool, is_halted: &mut bool) -> bool {
        *is_halted = true;

        true
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    reusable_testing_macro!(0x76, Halt);

    #[test]
    fn act_immehaltately_returns_true() {
        let mut registers = Registers::new(0, 0, 0, 0, 0, 0);
        let mut alu = ALU::new(registers.f.clone());

        let mut instruction = Halt {};

        let result = instruction.act(&mut registers, &mut alu, Arc::new(Mutex::new(MemoryController::new())) ,&mut false, &mut false);

        assert_eq!(true, result);
    }

    #[test]
    fn act_haltsables_interrupts() {
        let mut registers = Registers::new(0, 0, 0, 0, 0, 0);
        let mut alu = ALU::new(registers.f.clone());

        let mut instruction = Halt {};
        let mut halted = false;

        instruction.act(&mut registers, &mut alu, Arc::new(Mutex::new(MemoryController::new())), &mut false, &mut halted);

        assert_eq!(true, halted);
    }
}
