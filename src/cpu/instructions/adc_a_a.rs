use crate::cpu::alu::ALU;
use crate::cpu::instructions::Instruction;
use crate::cpu::register::Register;
use crate::cpu::register8::Register8;
use crate::cpu::registers::Registers;
use crate::memory::MemoryController;
use parking_lot::Mutex;
use std::sync::Arc;

pub struct AdcAA {}


impl Instruction for AdcAA {

    #[inline]
    fn from_opcode(opcode: &u8) -> Option<Box<dyn Instruction>> {
        if *opcode == 0x8F {
            return Some(Box::new(AdcAA {}))
        }
        None
    }


    fn get_opcode(&self) -> u8 {
        0x8F
    }

    fn act(&mut self, registers: &mut Registers, alu: &mut ALU, _memory_controller: Arc<Mutex<MemoryController>>, _enable_interrupts: &mut bool, _is_halted: &mut bool) -> bool {
        let fake_register = Register8::new(registers.a.borrow().get_value());
        alu.adc(&mut *registers.a.clone().borrow_mut(), &fake_register);

        true
    }
}


#[cfg(test)]
mod add_a_a {
    use super::*;

    reusable_testing_macro!(0x8F, AdcAA);

    #[test]
    fn adc_r_to_a_on_tick_no_carry() {
        let mut registers = Registers::new(0, 0, 0, 0, 0xC000, 0);
        registers.a.borrow_mut().set_value(0x12);
        registers.f.borrow_mut().set_bit(ALU::CARRY_FLAG, false);

        let memory = Arc::new(Mutex::new(MemoryController::new()));
        let mut alu = ALU::new(registers.f.clone());

        let mut instruction = AdcAA {};

        let result = instruction.act(&mut registers, &mut alu, memory.clone(), &mut false, &mut false);

        assert_eq!(true, result);
        assert_eq!(0x24, registers.a.borrow().get_value());
    }

    #[test]
    fn adc_r_to_a_on_tick_with_carry() {
        let mut registers = Registers::new(0, 0, 0, 0, 0xC000, 0);
        registers.a.borrow_mut().set_value(0x12);
        registers.f.borrow_mut().set_bit(ALU::CARRY_FLAG, true);

        let memory = Arc::new(Mutex::new(MemoryController::new()));
        let mut alu = ALU::new(registers.f.clone());

        let mut instruction = AdcAA {};

        let result = instruction.act(&mut registers, &mut alu, memory.clone(), &mut false, &mut false);

        assert_eq!(true, result);
        assert_eq!(0x25, registers.a.borrow().get_value());
    }
}
