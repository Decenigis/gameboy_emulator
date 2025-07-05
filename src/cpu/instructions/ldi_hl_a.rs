use std::sync::Arc;
use parking_lot::Mutex;
use crate::cpu::alu::ALU;
use crate::cpu::instructions::Instruction;
use crate::cpu::register::Register;
use crate::cpu::registers::Registers;
use crate::memory::{MemoryController, MemoryTrait};

pub struct LdiHlA {
    counter: u8
}

impl Instruction for LdiHlA {

    #[inline]
    fn from_opcode(opcode: &u8) -> Option<Box<dyn Instruction>> {
        if *opcode == 0x22 {
            return Some(Box::new(LdiHlA { counter: 1 }))
        }
        None
    }

    fn get_opcode(&self) -> u8 {
        0x22
    }

    fn act(&mut self, registers: &mut Registers, _alu: &mut ALU, memory_controller: Arc<Mutex<MemoryController>>, _enable_interrupts: &mut bool) -> bool {
        if self.counter == 1 {
            memory_controller.lock().set(registers.hl.get_value(), registers.a.borrow().get_value());
        }
        else if self.counter == 0 {
            registers.hl.increment();

            return true;
        }

        self.counter -= 1;
        false
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    reusable_testing_macro!(0x22, LdiHlA);

    #[test]
    fn load_value_to_hl_on_tick_1() {
        let mut registers = Registers::new(0, 0, 0, 0xC000, 0xC000, 0);
        let mut alu = ALU::new(registers.f.clone());
        let memory = Arc::new(Mutex::new(MemoryController::new()));

        registers.a.borrow_mut().set_value(0x12);

        let mut instruction = LdiHlA { counter: 1 };

        let result = instruction.act(&mut registers, &mut alu, memory.clone(),&mut false);

        assert_eq!(false, result);
        assert_eq!(0x12, memory.lock().get(0xC000));
    }

    #[test]
    fn increment_hl_on_tick_0_and_get_next_instruction() {
        let mut registers = Registers::new(0, 0, 0, 0xC000, 0xC000, 0);
        let mut alu = ALU::new(registers.f.clone());
        let memory = Arc::new(Mutex::new(MemoryController::new()));

        let mut instruction = LdiHlA { counter: 0 };

        let result = instruction.act(&mut registers, &mut alu, memory.clone(),&mut false);

        assert_eq!(true, result);
        assert_eq!(0xC001, registers.hl.get_value());
    }
}
