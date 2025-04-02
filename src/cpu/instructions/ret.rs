use std::sync::Arc;
use parking_lot::Mutex;
use crate::cpu::alu::ALU;
use crate::cpu::instructions::Instruction;
use crate::cpu::register::Register;
use crate::cpu::registers::Registers;
use crate::memory::{MemoryController, MemoryTrait};

pub struct Ret {
    counter: u8,
    address_low_byte: u8,
}

impl Instruction for Ret {

    fn from_opcode(opcode: &u8) -> Option<Box<dyn Instruction>> {
        if *opcode == 0xC9 {
            return Some(Box::new(Ret { counter: 1, address_low_byte: 0 }))
        }
        None
    }

    fn get_opcode(&self) -> u8 {
        0xC9
    }

    fn act(&mut self, registers: &mut Registers, _alu: &mut ALU, memory_controller: Arc<Mutex<MemoryController>>, enable_interrupts: &mut bool) -> bool {
        if self.counter == 1 {
            self.address_low_byte = memory_controller.lock().get(registers.sp.get_value());
            registers.sp.increment();
        }
        else if self.counter == 0 {
            let address_high_byte = memory_controller.lock().get(registers.sp.get_value());
            registers.sp.increment();

            registers.pc.set_value((address_high_byte as u16) << 8 | (self.address_low_byte as u16));

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
    fn from_opcode_returns_given_0xc9() {
        let instruction = Ret::from_opcode(&0xC9);

        assert_eq!(true, instruction.is_some());
    }

    #[test]
    fn from_opcode_returns_none_given_non_0xc9() {
        let instruction = Ret::from_opcode(&0x00);

        assert_eq!(true, instruction.is_none());
    }

    #[test]
    fn get_opcode_returns_0xc9() {
        let instruction = Ret { counter: 0, address_low_byte: 0 };

        assert_eq!(0xC9, instruction.get_opcode());
    }

    #[test]
    fn get_low_return_byte_on_clock_1() {
        let mut registers = Registers::new(0, 0, 0, 0, 0, 0xC000);
        let mut alu = ALU::new(registers.f.clone());
        let memory = Arc::new(Mutex::new(MemoryController::new()));

        memory.lock().set(0xC000, 0x12);

        let mut instruction = Ret { counter: 1, address_low_byte: 0 };

        let result = instruction.act(&mut registers, &mut alu, memory,&mut false);

        assert_eq!(false, result);
        assert_eq!(0x12, instruction.address_low_byte);
        assert_eq!(0xC001, registers.sp.get_value());
    }

    #[test]
    fn set_pc_on_clock_0_and_get_next_instruction() {
        let mut registers = Registers::new(0, 0, 0, 0, 0, 0xC001);
        let mut alu = ALU::new(registers.f.clone());
        let memory = Arc::new(Mutex::new(MemoryController::new()));

        memory.lock().set(0xC001, 0x34);

        let mut instruction = Ret { counter: 0, address_low_byte: 0x12 };

        let result = instruction.act(&mut registers, &mut alu, memory,&mut false);

        assert_eq!(true, result);
        assert_eq!(0x3412, registers.pc.get_value());
        assert_eq!(0xC002, registers.sp.get_value());
    }
}
