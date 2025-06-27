use std::sync::Arc;
use parking_lot::Mutex;
use crate::cpu::alu::ALU;
use crate::cpu::instructions::Instruction;
use crate::cpu::register::Register;
use crate::cpu::registers::Registers;
use crate::memory::{MemoryController, MemoryTrait};

//Timings for this one might be a bit off due to te 4-cycle alignment

pub struct LdNNA {
    counter: u8,
    address: u16,
}

impl Instruction for LdNNA {

    #[inline]
    fn from_opcode(opcode: &u8) -> Option<Box<dyn Instruction>> {
        if *opcode == 0xE0 {
            return Some(Box::new(LdNNA { counter: 3, address: 0 }))
        }
        None
    }

    fn get_opcode(&self) -> u8 {
        0xE0
    }

    fn act(&mut self, registers: &mut Registers, _alu: &mut ALU, memory_controller: Arc<Mutex<MemoryController>>, _enable_interrupts: &mut bool) -> bool {
        if self.counter == 3 {
            self.address = memory_controller.lock().get(registers.pc.get_value()) as u16;
            registers.pc.increment();
        }
        else if self.counter == 2 {
            self.address = self.address | (memory_controller.lock().get(registers.pc.get_value()) as u16 >> 8);
            registers.pc.increment();
        }
        else if self.counter == 1 {
            memory_controller.lock().set(self.address, registers.a.get_value());
            registers.pc.increment();
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
    use crate::cpu::register::Register;
    use crate::memory::MemoryTrait;
    use super::*;

    #[test]
    fn from_opcode_returns_given_0xe0() {
        let instruction = LdNNA::from_opcode(&0xE0);

        assert_eq!(true, instruction.is_some());
    }

    #[test]
    fn from_opcode_returns_none_given_non_0xe0() {
        let instruction = LdNNA::from_opcode(&0x00);

        assert_eq!(true, instruction.is_none());
    }

    #[test]
    fn get_opcode_returns_0xe0() {
        let instruction = LdNNA { counter: 0, address: 0 };

        assert_eq!(0xE0, instruction.get_opcode());
    }


    #[test]
    fn load_address_on_tick_2() {
        let mut registers = Registers::new(0, 0, 0, 0, 0xC000, 0);
        let mut alu = ALU::new(registers.f.clone());
        let memory = Arc::new(Mutex::new(MemoryController::new()));

        memory.lock().set(0xC000, 0x12);

        let mut instruction = LdNNA { counter: 2, address: 0 };

        let result = instruction.act(&mut registers, &mut alu, memory.clone(),&mut false);

        assert_eq!(false, result);

        assert_eq!(0xFF12, instruction.address);
    }

    #[test]
    fn load_data_into_a_on_tick_1() {
        let expected_a_value = 0x12;

        let mut registers = Registers::new(0, 0, 0, 0, 0xC000, 0);
        registers.a.borrow_mut().set_value(expected_a_value);

        let mut alu = ALU::new(registers.f.clone());
        let memory = Arc::new(Mutex::new(MemoryController::new()));

        memory.lock().set(0xFF40, 0x00);

        let mut instruction = LdNNA { counter: 1, address: 0xFF40 };

        let result = instruction.act(&mut registers, &mut alu, memory.clone(),&mut false);

        assert_eq!(false, result);

        assert_eq!(expected_a_value, memory.lock().get(0xFF40));
    }

    #[test]
    fn get_next_instruction_on_tick_0() {
        let mut registers = Registers::new(0, 0, 0, 0, 0xC000, 0);
        let mut alu = ALU::new(registers.f.clone());
        let memory = Arc::new(Mutex::new(MemoryController::new()));

        memory.lock().set(0xC000, 0x12);

        let mut instruction = LdNNA { counter: 0, address: 0x1234 };

        let result = instruction.act(&mut registers, &mut alu, memory.clone(),&mut false);

        assert_eq!(true, result);
    }
}
