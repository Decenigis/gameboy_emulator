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
        if *opcode == 0xEA {
            return Some(Box::new(LdNNA { counter: 3, address: 0 }))
        }
        None
    }

    fn get_opcode(&self) -> u8 {
        0xEA
    }

    fn act(&mut self, registers: &mut Registers, _alu: &mut ALU, memory_controller: Arc<Mutex<MemoryController>>, _enable_interrupts: &mut bool) -> bool {
        if self.counter == 3 {
            self.address = memory_controller.lock().get(registers.pc.get_value()) as u16;
            registers.pc.increment();
        }
        else if self.counter == 2 {
            self.address = self.address | ((memory_controller.lock().get(registers.pc.get_value()) as u16) << 8);
            registers.pc.increment();
        }
        else if self.counter == 1 {
            memory_controller.lock().set(self.address, registers.a.borrow().get_value());
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

    reusable_testing_macro!(0xEA, LdNNA);

    #[test]
    fn load_address_on_tick_3() {
        let mut registers = Registers::new(0, 0, 0, 0, 0xC000, 0);
        let mut alu = ALU::new(registers.f.clone());
        let memory = Arc::new(Mutex::new(MemoryController::new()));

        memory.lock().set(0xC000, 0x12);

        let mut instruction = LdNNA { counter: 3, address: 0 };

        let result = instruction.act(&mut registers, &mut alu, memory.clone(),&mut false);

        assert_eq!(false, result);

        assert_eq!(0x12, instruction.address);
        assert_eq!(0xC001, registers.pc.get_value());
    }

    #[test]
    fn load_address_on_tick_2() {
        let mut registers = Registers::new(0, 0, 0, 0, 0xC001, 0);
        let mut alu = ALU::new(registers.f.clone());
        let memory = Arc::new(Mutex::new(MemoryController::new()));

        memory.lock().set(0xC001, 0xFF);

        let mut instruction = LdNNA { counter: 2, address: 0x12 };

        let result = instruction.act(&mut registers, &mut alu, memory.clone(),&mut false);

        assert_eq!(false, result);

        assert_eq!(0xFF12, instruction.address);
        assert_eq!(0xC002, registers.pc.get_value());
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
