use std::sync::Arc;
use parking_lot::Mutex;
use crate::cpu::alu::ALU;
use crate::cpu::instructions::Instruction;
use crate::cpu::register::Register;
use crate::cpu::registers::Registers;
use crate::memory::{MemoryController, MemoryTrait};

pub struct JP {
    counter: u8,
    address: u16
}

impl Instruction for JP {

    fn from_opcode(opcode: &u8) -> Option<Box<dyn Instruction>> {
        if *opcode == 0xC3 {
            return Some(Box::new(JP { counter: 2, address: 0 }))
        }
        None
    }

    fn get_opcode(&self) -> u8 {
        0xC3
    }

    fn act(&mut self, registers: &mut Registers, _alu: &mut ALU, memory_controller: Arc<Mutex<MemoryController>>, _enable_interrupts: &mut bool) -> bool {
        if self.counter == 2 {
            self.address = memory_controller.lock().get(registers.pc.get_value()) as u16;
            registers.pc.increment();
        }
        else if self.counter == 1 {
            self.address = self.address | ((memory_controller.lock().get(registers.pc.get_value()) as u16) << 8);
            registers.pc.increment();
        }
        else if self.counter == 0 {
            registers.pc.set_value(self.address);
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
    fn from_opcode_returns_given_0xc3() {
        let instruction = JP::from_opcode(&0xC3);

        assert_eq!(true, instruction.is_some());
    }

    #[test]
    fn from_opcode_returns_none_given_non_0xc3() {
        let instruction = JP::from_opcode(&0x00);

        assert_eq!(true, instruction.is_none());
    }

    #[test]
    fn get_opcode_returns_0xc3() {
        let instruction = JP { counter: 0, address: 0 };

        assert_eq!(0xC3, instruction.get_opcode());
    }

    #[test]
    fn act_loads_new_address_over_3_ticks() {
        let mut registers = Registers::new(0, 0, 0, 0, 0xC000, 0);
        let mut alu = ALU::new(registers.f.clone());
        let memory = Arc::new(Mutex::new(MemoryController::new()));

        memory.lock().set(0xC000, 0x34);
        memory.lock().set(0xC001, 0x12);

        let mut instruction = JP { counter: 2, address: 0 };

        let result1 = instruction.act(&mut registers, &mut alu, memory.clone(),&mut false);
        let result2 = instruction.act(&mut registers, &mut alu, memory.clone(),&mut false);
        let result3 = instruction.act(&mut registers, &mut alu, memory.clone(),&mut false);

        assert_eq!(false, result1);
        assert_eq!(false, result2);
        assert_eq!(true, result3);

        assert_eq!(0x1234, registers.pc.get_value());
    }
}
