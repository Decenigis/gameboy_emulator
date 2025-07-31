use crate::cpu::alu::ALU;
use crate::cpu::instructions::Instruction;
use crate::cpu::registers::Registers;
use crate::memory::MemoryController;
use parking_lot::Mutex;
use std::sync::Arc;
use crate::cpu::register::Register;

pub struct Scf {}

impl Instruction for Scf {

    #[inline]
    fn from_opcode(opcode: &u8) -> Option<Box<dyn Instruction>> {
        if *opcode == 0x37 {
            return Some(Box::new(Scf {}))
        }
        None
    }

    fn get_opcode(&self) -> u8 {
        0x37
    }

    fn act(&mut self, registers: &mut Registers, _alu: &mut ALU, _memory_controller: Arc<Mutex<MemoryController>>, _enable_interrupts: &mut bool, _is_halted: &mut bool) -> bool {
        registers.f.borrow_mut().set_bit(ALU::SUB_FLAG, false);
        registers.f.borrow_mut().set_bit(ALU::HALF_CARRY_FLAG, false);
        registers.f.borrow_mut().set_bit(ALU::CARRY_FLAG, true);
        
        true
    }
}



#[cfg(test)]
mod tests {
    use crate::cpu::register::Register;
    use super::*;

    reusable_testing_macro!(0x37, Scf);

    #[test]
    fn act_immediately_returns_true() {
        let mut registers = Registers::new(0, 0, 0, 0, 0, 0);
        let mut alu = ALU::new(registers.f.clone());

        let mut instruction = Scf {};

        let result = instruction.act(&mut registers, &mut alu, Arc::new(Mutex::new(MemoryController::new())) ,&mut false, &mut false);

        assert_eq!(true, result);
    }

    #[test]
    fn act_sets_flags() {
        let mut registers = Registers::new(0, 0, 0, 0, 0, 0);
        let mut alu = ALU::new(registers.f.clone());

        let mut instruction = Scf {};
        let mut enable_interrupts = false;

        instruction.act(&mut registers, &mut alu, Arc::new(Mutex::new(MemoryController::new())), &mut enable_interrupts, &mut false);

        assert_eq!(false, registers.f.borrow().get_bit(ALU::SUB_FLAG));
        assert_eq!(false, registers.f.borrow().get_bit(ALU::HALF_CARRY_FLAG));
        assert_eq!(true, registers.f.borrow().get_bit(ALU::CARRY_FLAG));
    }
}
