use std::sync::Arc;
use parking_lot::Mutex;
use crate::cpu::alu::ALU;
use crate::cpu::instructions::Instruction;
use crate::cpu::register::Register;
use crate::cpu::registers::Registers;
use crate::memory::{MemoryController, MemoryTrait};

pub struct LdAN {
    counter: u8
}

impl Instruction for LdAN {

    #[inline]
    fn from_opcode(opcode: &u8) -> Option<Box<dyn Instruction>> {
        if *opcode == 0x3E {
            return Some(Box::new(LdAN { counter: 1 }))
        }
        None
    }

    fn get_opcode(&self) -> u8 {
        0x3E
    }

    fn act(&mut self, registers: &mut Registers, _alu: &mut ALU, memory_controller: Arc<Mutex<MemoryController>>, _enable_interrupts: &mut bool) -> bool {
        if self.counter == 1 {
            registers.a.borrow_mut().set_value(memory_controller.lock().get(registers.pc.get_value()));
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
    use super::*;

    #[test]
    fn from_opcode_returns_given_0x3e() {
        let instruction = LdAN::from_opcode(&0x3E);

        assert_eq!(true, instruction.is_some());
    }

    #[test]
    fn from_opcode_returns_none_given_non_0x3e() {
        let instruction = LdAN::from_opcode(&0x00);

        assert_eq!(true, instruction.is_none());
    }

    #[test]
    fn get_opcode_returns_0x3e() {
        let instruction = LdAN { counter: 0 };

        assert_eq!(0x3E, instruction.get_opcode());
    }

    #[test]
    fn load_n_into_a_on_tick_1() {
        let mut registers = Registers::new(0, 0, 0, 0, 0xC000, 0);
        let mut alu = ALU::new(registers.f.clone());
        let memory = Arc::new(Mutex::new(MemoryController::new()));

        memory.lock().set(0xC000, 0x12);

        let mut instruction = LdAN { counter: 1 };

        let result = instruction.act(&mut registers, &mut alu, memory.clone(),&mut false);

        assert_eq!(false, result);

        assert_eq!(0x12, registers.a.borrow().get_value());
    }

    #[test]
    fn on_tick_0_get_next_instruction() {
        let mut registers = Registers::new(0, 0, 0, 0xD000, 0xC000, 0);
        let mut alu = ALU::new(registers.f.clone());
        let memory = Arc::new(Mutex::new(MemoryController::new()));

        let mut instruction = LdAN { counter: 0 };

        let result = instruction.act(&mut registers, &mut alu, memory.clone(),&mut false);

        assert_eq!(true, result);
    }
}
