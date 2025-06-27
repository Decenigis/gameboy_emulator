use std::sync::Arc;
use parking_lot::Mutex;
use crate::cpu::alu::ALU;
use crate::cpu::instructions::Instruction;
use crate::cpu::register::Register;
use crate::cpu::registers::Registers;
use crate::memory::{MemoryController, MemoryTrait};

//Timings for this one might be a bit off due to te 4-cycle alignment

pub struct LdhCA {
    counter: u8,
}

impl Instruction for LdhCA {

    #[inline]
    fn from_opcode(opcode: &u8) -> Option<Box<dyn Instruction>> {
        if *opcode == 0xE2 {
            return Some(Box::new(LdhCA { counter: 1 }))
        }
        None
    }

    fn get_opcode(&self) -> u8 {
        0xE2
    }

    fn act(&mut self, registers: &mut Registers, _alu: &mut ALU, memory_controller: Arc<Mutex<MemoryController>>, _enable_interrupts: &mut bool) -> bool {
        if self.counter == 1 {
            memory_controller.lock().set(0xFF00 | (registers.c.borrow().get_value() as u16), registers.a.borrow().get_value());
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

    reusable_testing_macro!(0xE2, LdhCA);


    #[test]
    fn load_data_into_a_on_tick_1() {
        let expected_a_value = 0x12;

        let mut registers = Registers::new(0, 0x0040, 0, 0, 0xC000, 0);
        registers.a.borrow_mut().set_value(expected_a_value);

        let mut alu = ALU::new(registers.f.clone());
        let memory = Arc::new(Mutex::new(MemoryController::new()));

        memory.lock().set(0xFF40, 0x00);

        let mut instruction = LdhCA { counter: 1 };

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

        let mut instruction = LdhCA { counter: 0 };

        let result = instruction.act(&mut registers, &mut alu, memory.clone(),&mut false);

        assert_eq!(true, result);
    }
}
