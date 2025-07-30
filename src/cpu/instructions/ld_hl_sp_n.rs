use crate::cpu::alu::ALU;
use crate::cpu::instructions::Instruction;
use crate::cpu::register::Register;
use crate::cpu::registers::Registers;
use crate::memory::{MemoryController, MemoryTrait};
use parking_lot::Mutex;
use std::sync::Arc;
use crate::cpu::register16::Register16;

pub struct LdHlSpN {
    counter: u8,
    value: u8
}

impl Instruction for LdHlSpN {

    #[inline]
    fn from_opcode(opcode: &u8) -> Option<Box<dyn Instruction>> {
        if *opcode == 0xF8 {
            return Some(Box::new(LdHlSpN { counter: 2, value: 0 }))
        }
        None
    }

    fn get_opcode(&self) -> u8 {
        0xF8
    }

    fn act(&mut self, registers: &mut Registers, alu: &mut ALU, memory_controller: Arc<Mutex<MemoryController>>, _enable_interrupts: &mut bool, _is_halted: &mut bool) -> bool {
        if self.counter == 2 {
            self.value = memory_controller.lock().get(registers.pc.get_value()) as u8;
            registers.pc.increment();
        }
        if self.counter == 1 {
            registers.hl.set_value(registers.sp.get_value());

            if self.value < 0x80 {
                alu.add(&mut registers.hl, &Register16::new(self.value as u16))
            } else {
                alu.sub(&mut registers.hl, &Register16::new(0x100 - (self.value as u16)))
            }

            registers.f.borrow_mut().set_bit(ALU::ZERO_FLAG, false);
            registers.f.borrow_mut().set_bit(ALU::SUB_FLAG, false);
        }
        else if self.counter == 0 {
            return true;
        }

        self.counter -= 1;
        false
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    reusable_testing_macro!(0xF8, LdHlSpN);

    #[test]
    fn load_byte_on_tick_2() {
        let mut registers = Registers::new(0, 0, 0, 0x1234, 0xC000, 0);
        let mut alu = ALU::new(registers.f.clone());
        let memory = Arc::new(Mutex::new(MemoryController::new()));

        memory.lock().set(0xC000, 0x12);

        let mut instruction = LdHlSpN { counter: 1, value: 0 };

        let result = instruction.act(&mut registers, &mut alu, memory.clone(),&mut false, &mut false);

        assert_eq!(false, result);
        assert_eq!(0xC001, registers.pc.get_value());
        assert_eq!(0x12, instruction.value);
    }

    #[test]
    fn calculate_new_hl_value_for_addition_on_tick_1() {
        let mut registers = Registers::new(0, 0, 0, 0x1234, 0xC001, 0);
        let mut alu = ALU::new(registers.f.clone());
        let memory = Arc::new(Mutex::new(MemoryController::new()));

        registers.sp.set_value(0xC000);

        let mut instruction = LdHlSpN { counter: 1, value: 0x12 };

        let result = instruction.act(&mut registers, &mut alu, memory.clone(),&mut false, &mut false);

        assert_eq!(false, result);
        assert_eq!(0xC012, registers.hl.get_value());
        assert_eq!(0xC001, registers.pc.get_value());
    }

    #[test]
    fn calculate_new_hl_value_for_subtraction_on_tick_1() {
        let mut registers = Registers::new(0, 0, 0, 0x1234, 0xC001, 0);
        let mut alu = ALU::new(registers.f.clone());
        let memory = Arc::new(Mutex::new(MemoryController::new()));

        registers.sp.set_value(0xC000);

        let mut instruction = LdHlSpN { counter: 1, value: 0xEE };

        let result = instruction.act(&mut registers, &mut alu, memory.clone(),&mut false, &mut false);

        assert_eq!(false, result);
        assert_eq!(0xBFEE, registers.hl.get_value());
        assert_eq!(0xC001, registers.pc.get_value());
    }


    #[test]
    fn sets_correct_flags_on_tick_1() {
        let mut registers = Registers::new(0, 0, 0, 0x1234, 0xC001, 0);
        let mut alu = ALU::new(registers.f.clone());
        let memory = Arc::new(Mutex::new(MemoryController::new()));

        let mut instruction = LdHlSpN { counter: 1, value: 0xEE };

        let result = instruction.act(&mut registers, &mut alu, memory.clone(),&mut false, &mut false);

        assert_eq!(false, result);
        assert_eq!(false, registers.f.borrow().get_bit(ALU::ZERO_FLAG));
        assert_eq!(false, registers.f.borrow().get_bit(ALU::SUB_FLAG));
    }
    
    #[test]
    fn on_tick_0_and_get_next_instruction() {
        let mut registers = Registers::new(0, 0, 0, 0, 0xC001, 0);
        let mut alu = ALU::new(registers.f.clone());
        let memory = Arc::new(Mutex::new(MemoryController::new()));

        let mut instruction = LdHlSpN { counter: 0, value: 0 };

        let result = instruction.act(&mut registers, &mut alu, memory.clone(),&mut false, &mut false);

        assert_eq!(true, result);
    }
}
