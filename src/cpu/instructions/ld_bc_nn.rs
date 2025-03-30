use std::sync::Arc;
use parking_lot::Mutex;
use crate::cpu::alu::ALU;
use crate::cpu::instructions::Instruction;
use crate::cpu::register::Register;
use crate::cpu::registers::Registers;
use crate::memory::{MemoryController, MemoryTrait};

pub struct LdBcNn {
    counter: u8,
    value: u16
}

impl Instruction for LdBcNn {

    fn from_opcode(opcode: &u8) -> Option<Box<dyn Instruction>> {
        if *opcode == 0x01 {
            return Some(Box::new(LdBcNn { counter: 2, value: 0 }))
        }
        None
    }

    fn get_opcode(&self) -> u8 {
        0x01
    }

    fn act(&mut self, registers: &mut Registers, _alu: &mut ALU, memory_controller: Arc<Mutex<MemoryController>>, _enable_interrupts: &mut bool) -> bool {
        if self.counter == 2 {
            self.value = memory_controller.lock().get(registers.pc.get_value()) as u16;
            registers.pc.increment();
        }
        else if self.counter == 1 {
            self.value = self.value | ((memory_controller.lock().get(registers.pc.get_value()) as u16) << 8);
            registers.pc.increment();
        }
        else if self.counter == 0 {
            registers.bc.set_value(self.value);
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
    fn from_opcode_returns_given_0x01() {
        let instruction = LdBcNn::from_opcode(&0x01);

        assert_eq!(true, instruction.is_some());
    }

    #[test]
    fn from_opcode_returns_none_given_non_0x01() {
        let instruction = LdBcNn::from_opcode(&0x00);

        assert_eq!(true, instruction.is_none());
    }

    #[test]
    fn get_opcode_returns_0x01() {
        let instruction = LdBcNn { counter: 0, value: 0 };

        assert_eq!(0x01, instruction.get_opcode());
    }

    #[test]
    fn load_low_byte_on_tick_2() {
        let mut registers = Registers::new(0, 0, 0, 0, 0xC000, 0);
        let mut alu = ALU::new(registers.f.clone());
        let memory = Arc::new(Mutex::new(MemoryController::new()));

        memory.lock().set(0xC000, 0x12);

        let mut instruction = LdBcNn { counter: 2, value: 0 };

        let result = instruction.act(&mut registers, &mut alu, memory.clone(),&mut false);

        assert_eq!(false, result);

        assert_eq!(0x0012, instruction.value);
    }

    #[test]
    fn load_high_byte_on_tick_1() {
        let mut registers = Registers::new(0, 0, 0, 0, 0xC000, 0);
        let mut alu = ALU::new(registers.f.clone());
        let memory = Arc::new(Mutex::new(MemoryController::new()));

        memory.lock().set(0xC000, 0x12);

        let mut instruction = LdBcNn { counter: 1, value: 0 };

        let result = instruction.act(&mut registers, &mut alu, memory.clone(),&mut false);

        assert_eq!(false, result);

        assert_eq!(0x1200, instruction.value);
    }

    #[test]
    fn update_sp_on_tick_0_and_get_next_instruction() {
        let mut registers = Registers::new(0, 0, 0, 0, 0xC000, 0);
        let mut alu = ALU::new(registers.f.clone());
        let memory = Arc::new(Mutex::new(MemoryController::new()));

        let mut instruction = LdBcNn { counter: 0, value: 0x1234 };

        let result = instruction.act(&mut registers, &mut alu, memory.clone(),&mut false);

        assert_eq!(true, result);

        assert_eq!(0x1234, registers.bc.get_value());
    }
}
