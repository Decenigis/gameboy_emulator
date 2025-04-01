use std::sync::Arc;
use parking_lot::Mutex;
use crate::cpu::alu::ALU;
use crate::cpu::instructions::Instruction;
use crate::cpu::registers::Registers;
use crate::memory::MemoryController;

pub struct XorA {}

impl Instruction for XorA {

    fn from_opcode(opcode: &u8) -> Option<Box<dyn Instruction>> {
        if *opcode == 0xAF {
            return Some(Box::new(XorA {}))
        }
        None
    }

    fn get_opcode(&self) -> u8 {
        0xAF
    }

    fn act(&mut self, registers: &mut Registers, alu: &mut ALU, _memory_controller: Arc<Mutex<MemoryController>>, _enable_interrupts: &mut bool) -> bool {
        alu.xor_internal(registers.a.clone(), registers.a.clone());

        true
    }
}



#[cfg(test)]
mod tests {
    use crate::cpu::register::Register;
    use super::*;

    #[test]
    fn from_opcode_returns_given_0xaf() {
        let instruction = XorA::from_opcode(&0xAF);

        assert_eq!(true, instruction.is_some());
    }

    #[test]
    fn from_opcode_returns_none_given_non_0xaf() {
        let instruction = XorA::from_opcode(&0x00);

        assert_eq!(true, instruction.is_none());
    }

    #[test]
    fn get_opcode_returns_0xaf() {
        let instruction = XorA {};

        assert_eq!(0xAF, instruction.get_opcode());
    }

    #[test]
    fn act_immediately_returns_true() {
        let mut registers = Registers::new(0, 0, 0, 0, 0, 0);
        let mut alu = ALU::new(registers.f.clone());

        let mut instruction = XorA {};

        let result = instruction.act(&mut registers, &mut alu, Arc::new(Mutex::new(MemoryController::new())) ,&mut false);

        assert_eq!(true, result);
    }

    #[test]
    fn act_results_in_zero() {
        let mut registers = Registers::new(0xFF00, 0, 0, 0, 0, 0);
        let mut alu = ALU::new(registers.f.clone());

        let mut instruction = XorA {};

        instruction.act(&mut registers, &mut alu, Arc::new(Mutex::new(MemoryController::new())), &mut false);

        assert_eq!(0x00, registers.a.borrow().get_value());
    }
}
