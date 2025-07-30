use crate::cpu::alu::ALU;
use crate::cpu::instructions::Instruction;
use crate::cpu::register::Register;
use crate::cpu::registers::Registers;
use crate::memory::{MemoryController, MemoryTrait};
use parking_lot::Mutex;
use std::sync::Arc;

pub struct LddAHl {
    counter: u8
}

impl Instruction for LddAHl {

    #[inline]
    fn from_opcode(opcode: &u8) -> Option<Box<dyn Instruction>> {
        if *opcode == 0x3A {
            return Some(Box::new(LddAHl { counter: 1 }))
        }
        None
    }

    fn get_opcode(&self) -> u8 {
        0x3A
    }

    fn act(&mut self, registers: &mut Registers, _alu: &mut ALU, memory_controller: Arc<Mutex<MemoryController>>, _enable_interrupts: &mut bool, _is_halted: &mut bool) -> bool {
        if self.counter == 1 {
            registers.a.borrow_mut().set_value(memory_controller.lock().get(registers.hl.get_value()));
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

    reusable_testing_macro!(0x3A, LddAHl);

    #[test]
    fn load_n_on_tick_1() {
        let mut registers = Registers::new(0, 0, 0, 0xC000, 0xC000, 0);
        let mut alu = ALU::new(registers.f.clone());
        let memory = Arc::new(Mutex::new(MemoryController::new()));
        memory.lock().set(0xC000, 0x12);

        let mut instruction = LddAHl { counter: 1 };

        let result = instruction.act(&mut registers, &mut alu, memory.clone(),&mut false, &mut false);

        assert_eq!(false, result);
        assert_eq!(0x12, registers.a.borrow().get_value());
    }

    #[test]
    fn write_value_on_tick_0_and_get_next_instruction() {
        let mut registers = Registers::new(0, 0, 0, 0xC000, 0xC000, 0);
        let mut alu = ALU::new(registers.f.clone());
        let memory = Arc::new(Mutex::new(MemoryController::new()));

        let mut instruction = LddAHl { counter: 0 };

        let result = instruction.act(&mut registers, &mut alu, memory.clone(),&mut false, &mut false);

        assert_eq!(true, result);
        assert_eq!(0xC001, registers.hl.get_value());
    }
}
