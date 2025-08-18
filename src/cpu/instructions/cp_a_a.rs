use crate::cpu::alu::ALU;
use crate::cpu::instructions::Instruction;
use crate::cpu::registers::Registers;
use crate::memory::MemoryController;
use parking_lot::Mutex;
use std::sync::Arc;
use crate::cpu::register8::Register8;
use crate::cpu::register::Register;

pub struct CpAA {}

impl Instruction for CpAA {

    #[inline]
    fn from_opcode(opcode: &u8) -> Option<Box<dyn Instruction>> {
        if *opcode == 0xBF {
            return Some(Box::new(CpAA {}))
        }
        None
    }

    fn get_opcode(&self) -> u8 {
        0xBF
    }

    fn act(&mut self, registers: &mut Registers, alu: &mut ALU, _memory_controller: Arc<Mutex<MemoryController>>, _enable_interrupts: &mut bool, _is_halted: &mut bool) -> bool {
        alu.sub(&mut Register8::new(registers.a.borrow().get_value()), &registers.a.borrow());

        true
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpu::register::Register;

    reusable_testing_macro!(0xBF, CpAA);

    #[test]
    fn act_immediately_returns_true() {
        let mut registers = Registers::new(0, 0, 0, 0, 0, 0);
        let mut alu = ALU::new(registers.f.clone());

        let mut instruction = CpAA {};

        let result = instruction.act(&mut registers, &mut alu, Arc::new(Mutex::new(MemoryController::new())) ,&mut false, &mut false);

        assert_eq!(true, result);
    }

    #[test]
    fn act_results_in_correct_flags() {
        let mut registers = Registers::new(0x000, 0, 0, 0, 0, 0);
        let mut alu = ALU::new(registers.f.clone());

        let mut instruction = CpAA {};

        for i in 0..0xFF {
            registers.a.borrow_mut().set_value(i);

            instruction.act(&mut registers, &mut alu, Arc::new(Mutex::new(MemoryController::new())), &mut false, &mut false);

            assert_eq!(true, registers.f.borrow().get_bit(ALU::ZERO_FLAG));
            assert_eq!(true, registers.f.borrow().get_bit(ALU::SUB_FLAG));
            assert_eq!(false, registers.f.borrow().get_bit(ALU::HALF_CARRY_FLAG));
            assert_eq!(false, registers.f.borrow().get_bit(ALU::CARRY_FLAG));
        }
    }
}
