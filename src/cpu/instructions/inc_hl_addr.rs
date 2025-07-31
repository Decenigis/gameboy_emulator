use std::borrow::BorrowMut;
use crate::cpu::alu::ALU;
use crate::cpu::instructions::Instruction;
use crate::cpu::register::Register;
use crate::cpu::registers::Registers;
use crate::memory::{MemoryController, MemoryTrait};
use parking_lot::Mutex;
use std::sync::Arc;
use crate::cpu::register8::Register8;

pub struct IncHlAddr {
    counter: u8,
    value_register: Register8
}

impl Instruction for IncHlAddr {

    #[inline]
    fn from_opcode(opcode: &u8) -> Option<Box<dyn Instruction>> {
        if *opcode == 0x34 {
            return Some(Box::new(IncHlAddr { counter: 2, value_register: Register8::new(0x00) }));
        }
        None
    }

    fn get_opcode(&self) -> u8 {
        0x34
    }

    fn act(&mut self, registers: &mut Registers, alu: &mut ALU, memory_controller: Arc<Mutex<MemoryController>>, _enable_interrupts: &mut bool, _is_halted: &mut bool) -> bool {
        if self.counter == 2 {
            self.value_register.set_value(memory_controller.lock().get(registers.hl.get_value()));
        }
        else if self.counter == 1 {
            alu.add_no_carry(&mut self.value_register, &Register8::one());
        }
        else if self.counter == 0 {
            memory_controller.lock().set(registers.hl.get_value(), self.value_register.get_value());
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

    reusable_testing_macro!(0x34, IncHlAddr);

    #[test]
    fn get_value_on_tick_2() {
        let mut registers = Registers::new(0, 0, 0, 0xC000, 0xC000, 0);
        let mut alu = ALU::new(registers.f.clone());
        let memory = Arc::new(Mutex::new(MemoryController::new()));

        memory.lock().set(0xC000, 0x34);
        
        let mut instruction = IncHlAddr { counter: 2, value_register: Register8::new(0x00) };

        let result = instruction.act(&mut registers, &mut alu, memory.clone(),&mut false, &mut false);

        assert_eq!(false, result);
        assert_eq!(0x34, instruction.value_register.get_value());
    }

    #[test]
    fn inc_value_on_tick_1() {
        let mut registers = Registers::new(0, 0, 0, 0xC000, 0xC000, 0);
        let mut alu = ALU::new(registers.f.clone());
        let memory = Arc::new(Mutex::new(MemoryController::new()));

        let mut instruction = IncHlAddr { counter: 1, value_register: Register8::new(0x34) };

        let result = instruction.act(&mut registers, &mut alu, memory.clone(),&mut false, &mut false);

        assert_eq!(false, result);
        assert_eq!(0x35, instruction.value_register.get_value());
    }

    #[test]
    fn get_next_instruction_on_tick_0() {
        let mut registers = Registers::new(0, 0, 0, 0xC000, 0xC000, 0);
        let mut alu = ALU::new(registers.f.clone());
        let memory = Arc::new(Mutex::new(MemoryController::new()));

        let mut instruction = IncHlAddr { counter: 0, value_register: Register8::new(0x35) };

        let result = instruction.act(&mut registers, &mut alu, memory.clone(),&mut false, &mut false);

        assert_eq!(true, result);
        assert_eq!(0x35, memory.lock().get(registers.hl.get_value()));
    }
}
