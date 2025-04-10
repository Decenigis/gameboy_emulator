use std::sync::Arc;
use parking_lot::Mutex;
use crate::cpu::alu::ALU;
use crate::cpu::instructions::instruction::Instruction;
use crate::cpu::registers::Registers;
use crate::memory::MemoryController;

pub struct Nop {}

impl Instruction for Nop {

    #[inline]
    fn from_opcode(opcode: &u8) -> Option<Box<dyn Instruction>> {
        if *opcode == 0x00 {
            return Some(Box::new(Nop {}))
        }
        None
    }

    fn get_opcode(&self) -> u8 {
        0x00
    }

    fn act(&mut self, _registers: &mut Registers, _alu: &mut ALU, _memory_controller: Arc<Mutex<MemoryController>>, _enable_interrupts: &mut bool) -> bool {
        true
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_opcode_returns_given_0x00() {
        let instruction = Nop::from_opcode(&0x00);

        assert_eq!(true, instruction.is_some());
    }

    #[test]
    fn from_opcode_returns_none_given_non_0x00() {
        let instruction = Nop::from_opcode(&0x01);

        assert_eq!(true, instruction.is_none());
    }

    #[test]
    fn get_opcode_returns_0x00() {
        let instruction = Nop {};

        assert_eq!(0x00, instruction.get_opcode());
    }

    #[test]
    fn act_immediately_returns_true() {
        let mut registers = Registers::new(0, 0, 0, 0, 0, 0);
        let mut alu = ALU::new(registers.f.clone());

        let mut instruction = Nop {};

        let result = instruction.act(&mut registers, &mut alu, Arc::new(Mutex::new(MemoryController::new())) ,&mut false);

        assert_eq!(true, result);
    }
}
