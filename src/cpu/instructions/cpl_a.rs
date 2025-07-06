use crate::cpu::alu::ALU;
use crate::cpu::instructions::Instruction;
use crate::cpu::registers::Registers;
use crate::memory::MemoryController;
use parking_lot::Mutex;
use std::sync::Arc;
use crate::cpu::register::Register;

pub struct CplA {}

impl Instruction for CplA {

    #[inline]
    fn from_opcode(opcode: &u8) -> Option<Box<dyn Instruction>> {
        if *opcode == 0x2F {
            return Some(Box::new(CplA {}))
        }
        None
    }

    fn get_opcode(&self) -> u8 {
        0x2F
    }

    fn act(&mut self, registers: &mut Registers, _alu: &mut ALU, _memory_controller: Arc<Mutex<MemoryController>>, _enable_interrupts: &mut bool, _is_halted: &mut bool) -> bool {
        let new_value = !registers.a.borrow().get_value();
        registers.a.borrow_mut().set_value(new_value);

        let mut flags = registers.f.borrow_mut();
        flags.set_bit(ALU::HALF_CARRY_FLAG, true);
        flags.set_bit(ALU::SUB_FLAG, true);

        true
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpu::register::Register;

    reusable_testing_macro!(0x2F, CplA);
    #[test]
    fn act_immediately_returns_true() {
        let mut registers = Registers::new(0, 0, 0, 0, 0, 0);
        let mut alu = ALU::new(registers.f.clone());

        let mut instruction = CplA {};

        let result = instruction.act(&mut registers, &mut alu, Arc::new(Mutex::new(MemoryController::new())) ,&mut false, &mut false);

        assert_eq!(true, result);
    }

    #[test]
    fn act_flips_a() {
        let mut registers = Registers::new(0, 0, 0, 0, 0, 0);
        registers.a.borrow_mut().set_value(0xC3);

        let mut alu = ALU::new(registers.f.clone());

        let mut instruction = CplA {};

        instruction.act(&mut registers, &mut alu, Arc::new(Mutex::new(MemoryController::new())), &mut false, &mut false);

        assert_eq!(0x3C, registers.a.borrow().get_value());
        assert_eq!(true, registers.f.borrow().get_bit(ALU::HALF_CARRY_FLAG));
        assert_eq!(true, registers.f.borrow().get_bit(ALU::SUB_FLAG));

    }
}
