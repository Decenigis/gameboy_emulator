use crate::cpu::register8::Register8;
use crate::cpu::alu::ALU;
use crate::cpu::instructions::Instruction;
use crate::cpu::register::Register;
use crate::cpu::registers::Registers;
use crate::memory::{MemoryController, MemoryTrait};
use parking_lot::Mutex;
use std::sync::Arc;


pub struct RrcHl {
    counter: u8,
    value_register: Register8
}

impl Instruction for RrcHl {
    #[inline]
    fn from_opcode(opcode: &u8) -> Option<Box<dyn Instruction>> {
        if *opcode == 0x0E {
            return Some(Box::new(RrcHl{ counter: 3, value_register: Register8::new(0) }))
        }
        None
    }

    fn get_opcode(&self) -> u8 {
        0x0E
    }

    fn act(&mut self, registers: &mut Registers, alu: &mut ALU, memory_controller: Arc<Mutex<MemoryController>>, _enable_interrupts: &mut bool, _is_halted: &mut bool) -> bool {
        if self.counter == 3 {
            self.value_register.set_value(memory_controller.lock().get(registers.hl.get_value()));
        }
        else if self.counter == 2 {
            alu.rrc(&mut self.value_register);
        }
        else if self.counter == 1 {
            memory_controller.lock().set(registers.hl.get_value(), self.value_register.get_value());
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
    use crate::cpu::instructions::rrca::RrcA;
    use super::*;

    reusable_testing_macro!(0x0E, RrcHl);

    #[test]
    fn tests_bit_when_set_on_tick_3() {
        let mut registers = Registers::new(0, 0, 0, 0xC000, 0xC000, 0);
        let memory = Arc::new(Mutex::new(MemoryController::new()));
        let mut alu = ALU::new(registers.f.clone());

        memory.lock().set(0xC000, 0x12);

        let mut instruction = RrcHl { counter: 3, value_register: Register8::new(0) };

        let result = instruction.act(&mut registers, &mut alu, memory.clone(), &mut false, &mut false);

        assert_eq!(false, result);
        assert_eq!(0x12, instruction.value_register.get_value());
    }

    #[test]
    fn set_bit_on_tick_2() {
        let mut registers = Registers::new(0, 0, 0, 0, 0xC000, 0);
        let memory = Arc::new(Mutex::new(MemoryController::new()));
        let mut alu = ALU::new(registers.f.clone());

        let mut instruction = RrcHl { counter: 2, value_register: Register8::new(0b00010001) };

        let result = instruction.act(&mut registers, &mut alu, memory.clone(), &mut false, &mut false);

        assert_eq!(false, result);
        assert_eq!(0b10001000, instruction.value_register.get_value());
        assert_eq!(true, registers.f.borrow().get_bit(ALU::CARRY_FLAG));
    }

    #[test]
    fn write_value_on_tick_1() {
        let mut registers = Registers::new(0, 0, 0, 0xC000, 0xC000, 0);
        let memory = Arc::new(Mutex::new(MemoryController::new()));
        let mut alu = ALU::new(registers.f.clone());

        let mut instruction = RrcHl { counter: 1, value_register: Register8::new(0x12) };

        let result = instruction.act(&mut registers, &mut alu, memory.clone(), &mut false, &mut false);

        assert_eq!(false, result);
        assert_eq!(0x12, memory.lock().get(registers.hl.get_value()));
    }

    #[test]
    fn get_next_instruction_on_tick_0() {
        let mut registers = Registers::new(0, 0, 0, 0xC000, 0xC000, 0);
        let memory = Arc::new(Mutex::new(MemoryController::new()));
        let mut alu = ALU::new(registers.f.clone());

        let mut instruction = RrcHl { counter: 0, value_register: Register8::new(0x12) };

        let result = instruction.act(&mut registers, &mut alu, memory.clone(), &mut false, &mut false);

        assert_eq!(true, result);
    }
}
