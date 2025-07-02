use std::sync::Arc;
use parking_lot::Mutex;
use crate::cpu::alu::ALU;
use crate::cpu::instructions::Instruction;
use crate::cpu::register8::Register8;
use crate::cpu::register::Register;
use crate::cpu::registers::Registers;
use crate::memory::{MemoryController, MemoryTrait};

pub struct SubAN {
    counter: u8,
    value: u8
}


impl Instruction for SubAN {

    #[inline]
    fn from_opcode(opcode: &u8) -> Option<Box<dyn Instruction>> {
        if *opcode == 0xD6 {
            return Some(Box::new(SubAN {
                counter: 1,
                value: 1
            }))
        }
        None
    }


    fn get_opcode(&self) -> u8 {
        0xD6
    }

    fn act(&mut self, registers: &mut Registers, alu: &mut ALU, memory_controller: Arc<Mutex<MemoryController>>, _enable_interrupts: &mut bool) -> bool {
        if self.counter == 1 {
            self.value = memory_controller.lock().get(registers.pc.get_value());
            registers.pc.increment();
        }
        else if self.counter == 0 {
            alu.sub(&mut *registers.a.clone().borrow_mut(), &Register8::new(self.value));
            return true;
        }

        self.counter -= 1;
        false
    }
}


#[cfg(test)]
mod add_a_a {
    use super::*;

    reusable_testing_macro!(0xD6, SubAN);

    #[test]
    fn sub_a_from_n_on_tick_1() {
        let mut registers = Registers::new(0x36, 0, 0, 0, 0xC000, 0);
        let memory = Arc::new(Mutex::new(MemoryController::new()));
        let mut alu = ALU::new(registers.f.clone());

        memory.lock().set(0xC000, 0x12);

        let mut instruction = SubAN { counter: 1, value:0 };

        let result = instruction.act(&mut registers, &mut alu, memory.clone(), &mut false);

        assert_eq!(false, result);
        assert_eq!(0x12, instruction.value);
        assert_eq!(0xC001, registers.pc.get_value());
    }

    #[test]
    fn sub_a_from_n_on_tick_0() {
        let mut registers = Registers::new(0x3600, 0, 0, 0, 0xC001, 0);
        let memory = Arc::new(Mutex::new(MemoryController::new()));
        let mut alu = ALU::new(registers.f.clone());

        let mut instruction = SubAN { counter: 0, value: 0x12 };

        let result = instruction.act(&mut registers, &mut alu, memory.clone(), &mut false);

        assert_eq!(true, result);
        assert_eq!(0x24, registers.a.borrow().get_value());
    }
}
