use crate::cpu::alu::ALU;
use crate::cpu::instructions::Instruction;
use crate::cpu::register::Register;
use crate::cpu::registers::Registers;
use crate::memory::{MemoryController, MemoryTrait};
use parking_lot::Mutex;
use std::sync::Arc;

pub struct LdHlN {
    counter: u8,
    value: u8
}

impl Instruction for LdHlN {

    #[inline]
    fn from_opcode(opcode: &u8) -> Option<Box<dyn Instruction>> {
        if *opcode == 0x36 {
            return Some(Box::new(LdHlN { counter: 2, value: 0 }))
        }
        None
    }

    fn get_opcode(&self) -> u8 {
        0x36
    }

    fn act(&mut self, registers: &mut Registers, _alu: &mut ALU, memory_controller: Arc<Mutex<MemoryController>>, _enable_interrupts: &mut bool, _is_halted: &mut bool) -> bool {
        if self.counter == 2 {
            self.value = memory_controller.lock().get(registers.pc.get_value());
            registers.pc.increment();
        }
        else if self.counter == 0 {
            memory_controller.lock().set(registers.hl.get_value(), self.value);
            return true;
        }

        self.counter -= 1;
        false
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_opcode_returns_given_0x36() {
        let instruction = LdHlN::from_opcode(&0x36);

        assert_eq!(true, instruction.is_some());
    }

    #[test]
    fn from_opcode_returns_none_given_non_0x36() {
        let instruction = LdHlN::from_opcode(&0x00);

        assert_eq!(true, instruction.is_none());
    }

    #[test]
    fn get_opcode_returns_0x36() {
        let instruction = LdHlN { counter: 0, value: 0 };

        assert_eq!(0x36, instruction.get_opcode());
    }

    #[test]
    fn load_n_on_tick_2() {
        let mut registers = Registers::new(0, 0, 0, 0, 0xC000, 0);
        let mut alu = ALU::new(registers.f.clone());
        let memory = Arc::new(Mutex::new(MemoryController::new()));

        memory.lock().set(0xC000, 0x12);

        let mut instruction = LdHlN { counter: 2, value: 0 };

        let result = instruction.act(&mut registers, &mut alu, memory.clone(),&mut false, &mut false);

        assert_eq!(false, result);

        assert_eq!(0x12, instruction.value);
    }

    #[test]
    fn do_nothing_on_tick_1() {
        let mut registers = Registers::new(0, 0, 0, 0, 0xC000, 0);
        let mut alu = ALU::new(registers.f.clone());
        let memory = Arc::new(Mutex::new(MemoryController::new()));

        let mut instruction = LdHlN { counter: 1, value: 0 };

        let result = instruction.act(&mut registers, &mut alu, memory.clone(),&mut false, &mut false);

        assert_eq!(false, result);
    }

    #[test]
    fn write_value_on_tick_0_and_get_next_instruction() {
        let mut registers = Registers::new(0, 0, 0, 0xD000, 0xC000, 0);
        let mut alu = ALU::new(registers.f.clone());
        let memory = Arc::new(Mutex::new(MemoryController::new()));

        let mut instruction = LdHlN { counter: 0, value: 0x12 };

        let result = instruction.act(&mut registers, &mut alu, memory.clone(),&mut false, &mut false);

        assert_eq!(true, result);

        assert_eq!(0x12, memory.lock().get(0xD000));
    }
}
