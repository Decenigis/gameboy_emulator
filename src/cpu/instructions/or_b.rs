use crate::cpu::alu::ALU;
use crate::cpu::instructions::Instruction;
use crate::cpu::registers::Registers;
use crate::memory::MemoryController;
use parking_lot::Mutex;
use std::sync::Arc;

pub struct OrB {}

impl Instruction for OrB {

    #[inline]
    fn from_opcode(opcode: &u8) -> Option<Box<dyn Instruction>> {
        if *opcode == 0xB0 {
            return Some(Box::new(OrB {}))
        }
        None
    }

    fn get_opcode(&self) -> u8 {
        0xB0
    }

    fn act(&mut self, registers: &mut Registers, alu: &mut ALU, _memory_controller: Arc<Mutex<MemoryController>>, _enable_interrupts: &mut bool, _is_halted: &mut bool) -> bool {
        alu.or_internal(registers.a.clone(), registers.c.clone());

        true
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpu::register::Register;

    #[test]
    fn from_opcode_returns_given_0xb0() {
        let instruction = OrB::from_opcode(&0xB0);

        assert_eq!(true, instruction.is_some());
    }

    #[test]
    fn from_opcode_returns_none_given_non_0xb0() {
        let instruction = OrB::from_opcode(&0x00);

        assert_eq!(true, instruction.is_none());
    }

    #[test]
    fn get_opcode_returns_0xb0() {
        let instruction = OrB {};

        assert_eq!(0xB0, instruction.get_opcode());
    }

    #[test]
    fn act_immediately_returns_true() {
        let mut registers = Registers::new(0, 0, 0, 0, 0, 0);
        let mut alu = ALU::new(registers.f.clone());

        let mut instruction = OrB {};

        let result = instruction.act(&mut registers, &mut alu, Arc::new(Mutex::new(MemoryController::new())) ,&mut false, &mut false);

        assert_eq!(true, result);
    }

    #[test]
    fn act_results_in_zero() {
        let mut registers = Registers::new(0, 0, 0, 0, 0, 0);
        registers.a.borrow_mut().set_value(0xF0);
        registers.c.borrow_mut().set_value(0x0F);
        let mut alu = ALU::new(registers.f.clone());

        let mut instruction = OrB {};

        instruction.act(&mut registers, &mut alu, Arc::new(Mutex::new(MemoryController::new())), &mut false, &mut false);

        assert_eq!(0xFF, registers.a.borrow().get_value());
    }
}
