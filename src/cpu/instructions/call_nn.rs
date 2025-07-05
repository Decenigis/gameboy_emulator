use crate::cpu::alu::ALU;
use crate::cpu::instructions::Instruction;
use crate::cpu::register::Register;
use crate::cpu::registers::Registers;
use crate::memory::{MemoryController, MemoryTrait};
use parking_lot::Mutex;
use std::sync::Arc;

pub struct CallNn {
    counter: u8,
    address: u16
}

impl Instruction for CallNn {

    #[inline]
    fn from_opcode(opcode: &u8) -> Option<Box<dyn Instruction>> {
        if *opcode == 0xCD {
            return Some(Box::new(CallNn { counter: 2, address: 0 }))
        }
        None
    }

    fn get_opcode(&self) -> u8 {
        0xCD
    }

    //maybe wrong ordering, shouldn't have too much bearing.
    fn act(&mut self, registers: &mut Registers, _alu: &mut ALU, memory_controller: Arc<Mutex<MemoryController>>, _enable_interrupts: &mut bool, _is_halted: &mut bool) -> bool {
        if self.counter == 2 {
            self.address = memory_controller.lock().get(registers.pc.get_value()) as u16;
            self.address = self.address | ((memory_controller.lock().get(registers.pc.get_value() + 1) as u16) << 8);

            registers.pc.increment();
            registers.pc.increment();
        }
        else if self.counter == 1 {
            registers.sp.decrement();
            memory_controller.lock().set(registers.sp.get_value(), ((registers.pc.get_value() & 0xFF00) >> 8) as u8);
            registers.sp.decrement();
            memory_controller.lock().set(registers.sp.get_value(), (registers.pc.get_value() & 0xFF) as u8);
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
    fn from_opcode_returns_given_0xcd() {
        let instruction = CallNn::from_opcode(&0xCD);

        assert_eq!(true, instruction.is_some());
    }

    #[test]
    fn from_opcode_returns_none_given_non_0xcd() {
        let instruction = CallNn::from_opcode(&0x00);

        assert_eq!(true, instruction.is_none());
    }

    #[test]
    fn get_opcode_returns_0xcd() {
        let instruction = CallNn { counter: 0, address: 0 };

        assert_eq!(0xCD, instruction.get_opcode());
    }

    #[test]
    fn load_address_on_tick_2() {
        let mut registers = Registers::new(0, 0, 0, 0, 0xC000, 0xE000);
        let mut alu = ALU::new(registers.f.clone());
        let memory = Arc::new(Mutex::new(MemoryController::new()));

        memory.lock().set(0xC000, 0x12);
        memory.lock().set(0xC001, 0x34);

        let mut instruction = CallNn { counter: 2, address: 0 };

        let result = instruction.act(&mut registers, &mut alu, memory.clone(),&mut false, &mut false);

        assert_eq!(false, result);

        assert_eq!(0x3412, instruction.address);
    }

    #[test]
    fn saves_address_to_stack_on_tick_1() {
        let mut registers = Registers::new(0, 0, 0, 0, 0x1234, 0xE000);
        let mut alu = ALU::new(registers.f.clone());
        let memory = Arc::new(Mutex::new(MemoryController::new()));

        let mut instruction = CallNn { counter: 1, address: 0 };

        let result = instruction.act(&mut registers, &mut alu, memory.clone(),&mut false, &mut false);

        assert_eq!(false, result);

        assert_eq!(0xDFFE, registers.sp.get_value());
        assert_eq!(0x34, memory.lock().get(registers.sp.get_value()));
        assert_eq!(0x12, memory.lock().get(registers.sp.get_value() + 1));
    }

    #[test]
    fn update_pc_on_tick_0_and_get_next_instruction() {
        let mut registers = Registers::new(0, 0, 0, 0, 0xC000, 0);
        let mut alu = ALU::new(registers.f.clone());
        let memory = Arc::new(Mutex::new(MemoryController::new()));

        let mut instruction = CallNn { counter: 0, address: 0x1234 };

        let result = instruction.act(&mut registers, &mut alu, memory.clone(),&mut false, &mut false);

        assert_eq!(true, result);

        assert_eq!(0x1234, registers.pc.get_value());
    }
}
