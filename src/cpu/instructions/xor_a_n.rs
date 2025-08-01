use crate::cpu::alu::ALU;
use crate::cpu::instructions::Instruction;
use crate::cpu::register::Register;
use crate::cpu::register8::Register8;
use crate::cpu::registers::Registers;
use crate::memory::{MemoryController, MemoryTrait};
use parking_lot::Mutex;
use std::sync::Arc;

pub struct XorAN {
    counter: u8,
    value: u8
}


impl Instruction for XorAN {

    #[inline]
    fn from_opcode(opcode: &u8) -> Option<Box<dyn Instruction>> {
        if *opcode == 0xEE {
            return Some(Box::new(XorAN {
                counter: 1,
                value: 1
            }))
        }
        None
    }


    fn get_opcode(&self) -> u8 {
        0xEE
    }

    fn act(&mut self, registers: &mut Registers, alu: &mut ALU, memory_controller: Arc<Mutex<MemoryController>>, _enable_interrupts: &mut bool, _is_halted: &mut bool) -> bool {
        if self.counter == 1 {
            self.value = memory_controller.lock().get(registers.pc.get_value());
            registers.pc.increment();
        }
        else if self.counter == 0 {
            alu.xor(&mut *registers.a.clone().borrow_mut(), &Register8::new(self.value));
            return true;
        }

        self.counter -= 1;
        false
    }
}


#[cfg(test)]
mod add_a_a {
    use super::*;

    reusable_testing_macro!(0xEE, XorAN);

    #[test]
    fn load_a_on_tick_1() {
        let mut registers = Registers::new(0x36, 0, 0, 0, 0xC000, 0);
        let memory = Arc::new(Mutex::new(MemoryController::new()));
        let mut alu = ALU::new(registers.f.clone());

        memory.lock().set(0xC000, 0x12);

        let mut instruction = XorAN { counter: 1, value: 0 };

        let result = instruction.act(&mut registers, &mut alu, memory.clone(), &mut false, &mut false);

        assert_eq!(false, result);
        assert_eq!(0x12, instruction.value);
        assert_eq!(0xC001, registers.pc.get_value());
    }

    #[test]
    fn xor_a_from_n_on_tick_0() {
        let mut registers = Registers::new(0x3600, 0, 0, 0, 0xC001, 0);
        let memory = Arc::new(Mutex::new(MemoryController::new()));
        let mut alu = ALU::new(registers.f.clone());

        let mut instruction = XorAN { counter: 0, value: 0x12 };

        let result = instruction.act(&mut registers, &mut alu, memory.clone(), &mut false, &mut false);

        assert_eq!(true, result);
        assert_eq!(0x24, registers.a.borrow().get_value());
    }
}
