use crate::cpu::alu::ALU;
use crate::cpu::instructions::Instruction;
use crate::cpu::register::Register;
use crate::cpu::registers::Registers;
use crate::memory::{MemoryController, MemoryTrait};
use parking_lot::Mutex;
use std::sync::Arc;

//Timings for this one might be a bit off due to te 4-cycle alignment

pub struct LdhAC {
    counter: u8,
}

impl Instruction for LdhAC {

    #[inline]
    fn from_opcode(opcode: &u8) -> Option<Box<dyn Instruction>> {
        if *opcode == 0xF2 {
            return Some(Box::new(LdhAC { counter: 1 }))
        }
        None
    }

    fn get_opcode(&self) -> u8 {
        0xF2
    }

    fn act(&mut self, registers: &mut Registers, _alu: &mut ALU, memory_controller: Arc<Mutex<MemoryController>>, _enable_interrupts: &mut bool, _is_halted: &mut bool) -> bool {
        if self.counter == 1 {
            registers.a.borrow_mut().set_value(memory_controller.lock().get(0xFF00 | (registers.c.borrow().get_value() as u16)));
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
    use crate::cpu::register::Register;
    use crate::memory::MemoryTrait;

    reusable_testing_macro!(0xF2, LdhAC);


    #[test]
    fn load_data_into_a_on_tick_1() {
        let expected_a_value = 0x12;

        let mut registers = Registers::new(0, 0x0040, 0, 0, 0xC000, 0);

        let mut alu = ALU::new(registers.f.clone());
        let memory = Arc::new(Mutex::new(MemoryController::new()));

        memory.lock().set(0xFF40, expected_a_value);

        let mut instruction = LdhAC { counter: 1 };

        let result = instruction.act(&mut registers, &mut alu, memory.clone(),&mut false, &mut false);

        assert_eq!(false, result);
        assert_eq!(expected_a_value, registers.a.borrow().get_value());
    }

    #[test]
    fn get_next_instruction_on_tick_0() {
        let mut registers = Registers::new(0, 0, 0, 0, 0xC000, 0);
        let mut alu = ALU::new(registers.f.clone());
        let memory = Arc::new(Mutex::new(MemoryController::new()));

        memory.lock().set(0xC000, 0x12);

        let mut instruction = LdhAC { counter: 0 };

        let result = instruction.act(&mut registers, &mut alu, memory.clone(),&mut false, &mut false);

        assert_eq!(true, result);
    }
}
