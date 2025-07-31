use crate::cpu::alu::ALU;
use crate::cpu::instructions::Instruction;
use crate::cpu::registers::Registers;
use crate::memory::MemoryController;
use parking_lot::Mutex;
use std::sync::Arc;
use crate::cpu::register::Register;

pub struct Ccf {}

impl Instruction for Ccf {

    #[inline]
    fn from_opcode(opcode: &u8) -> Option<Box<dyn Instruction>> {
        if *opcode == 0x3F {
            return Some(Box::new(Ccf {}))
        }
        None
    }

    fn get_opcode(&self) -> u8 {
        0x3F
    }

    fn act(&mut self, registers: &mut Registers, _alu: &mut ALU, _memory_controller: Arc<Mutex<MemoryController>>, _enable_interrupts: &mut bool, _is_halted: &mut bool) -> bool {
        let carry_flag = registers.f.borrow().get_bit(ALU::CARRY_FLAG);

        registers.f.borrow_mut().set_bit(ALU::SUB_FLAG, false);
        registers.f.borrow_mut().set_bit(ALU::HALF_CARRY_FLAG, false);
        registers.f.borrow_mut().set_bit(ALU::CARRY_FLAG, !carry_flag);

        true
    }
}



#[cfg(test)]
mod tests {
    use crate::cpu::register::Register;
    use super::*;

    reusable_testing_macro!(0x3F, Ccf);

    #[test]
    fn act_immediately_returns_true() {
        let mut registers = Registers::new(0, 0, 0, 0, 0, 0);
        let mut alu = ALU::new(registers.f.clone());

        let mut instruction = Ccf {};

        let result = instruction.act(&mut registers, &mut alu, Arc::new(Mutex::new(MemoryController::new())) ,&mut false, &mut false);

        assert_eq!(true, result);
    }

    #[test]
    fn act_flips_carry_flag_for_true() {
        let mut registers = Registers::new(0, 0, 0, 0, 0, 0);
        let mut alu = ALU::new(registers.f.clone());

        registers.f.borrow_mut().set_bit(ALU::CARRY_FLAG, true);

        let mut instruction = Ccf {};

        instruction.act(&mut registers, &mut alu, Arc::new(Mutex::new(MemoryController::new())), &mut false, &mut false);

        assert_eq!(false, registers.f.borrow().get_bit(ALU::SUB_FLAG));
        assert_eq!(false, registers.f.borrow().get_bit(ALU::HALF_CARRY_FLAG));
        assert_eq!(false, registers.f.borrow().get_bit(ALU::CARRY_FLAG));
    }

    #[test]
    fn act_flips_carry_flag_for_false() {
        let mut registers = Registers::new(0, 0, 0, 0, 0, 0);
        let mut alu = ALU::new(registers.f.clone());

        registers.f.borrow_mut().set_bit(ALU::CARRY_FLAG, false);

        let mut instruction = Ccf {};

        instruction.act(&mut registers, &mut alu, Arc::new(Mutex::new(MemoryController::new())), &mut false, &mut false);

        assert_eq!(false, registers.f.borrow().get_bit(ALU::SUB_FLAG));
        assert_eq!(false, registers.f.borrow().get_bit(ALU::HALF_CARRY_FLAG));
        assert_eq!(true, registers.f.borrow().get_bit(ALU::CARRY_FLAG));
    }
}
