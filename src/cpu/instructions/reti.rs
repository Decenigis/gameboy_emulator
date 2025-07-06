use crate::cpu::alu::ALU;
use crate::cpu::instructions::Instruction;
use crate::cpu::register::Register;
use crate::cpu::registers::Registers;
use crate::memory::{MemoryController, MemoryTrait};
use parking_lot::Mutex;
use std::sync::Arc;

pub struct Reti {
    counter: u8,
    address_low_byte: u8,
}

impl Instruction for Reti {

    #[inline]
    fn from_opcode(opcode: &u8) -> Option<Box<dyn Instruction>> {
        if *opcode == 0xD9 {
            return Some(Box::new(Reti { counter: 1, address_low_byte: 0 }))
        }
        None
    }

    fn get_opcode(&self) -> u8 {
        0xD9
    }

    fn act(&mut self, registers: &mut Registers, _alu: &mut ALU, memory_controller: Arc<Mutex<MemoryController>>, enable_interrupts: &mut bool, _is_halted: &mut bool) -> bool {
        if self.counter == 1 {
            self.address_low_byte = memory_controller.lock().get(registers.sp.get_value());
            registers.sp.increment();
        }
        else if self.counter == 0 {
            let address_high_byte = memory_controller.lock().get(registers.sp.get_value());
            registers.sp.increment();

            registers.pc.set_value((address_high_byte as u16) << 8 | (self.address_low_byte as u16));

            *enable_interrupts = true;

            return true;
        }


        self.counter -= 1;
        false
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    reusable_testing_macro!(0xD9, Reti);

    #[test]
    fn get_low_return_byte_on_clock_1() {
        let mut registers = Registers::new(0, 0, 0, 0, 0, 0xC000);
        let mut alu = ALU::new(registers.f.clone());
        let memory = Arc::new(Mutex::new(MemoryController::new()));

        memory.lock().set(0xC000, 0x12);

        let mut instruction = Reti { counter: 1, address_low_byte: 0 };

        let result = instruction.act(&mut registers, &mut alu, memory,&mut false, &mut false);

        assert_eq!(false, result);
        assert_eq!(0x12, instruction.address_low_byte);
        assert_eq!(0xC001, registers.sp.get_value());
    }

    #[test]
    fn set_pc_on_clock_0_and_get_next_instruction() {
        let mut registers = Registers::new(0, 0, 0, 0, 0, 0xC001);
        let mut alu = ALU::new(registers.f.clone());
        let memory = Arc::new(Mutex::new(MemoryController::new()));
        let mut enable_interrupts = false;

        memory.lock().set(0xC001, 0x34);

        let mut instruction = Reti { counter: 0, address_low_byte: 0x12 };

        let result = instruction.act(&mut registers, &mut alu, memory,&mut enable_interrupts, &mut false);

        assert_eq!(true, result);
        assert_eq!(true, enable_interrupts);
        assert_eq!(0x3412, registers.pc.get_value());
        assert_eq!(0xC002, registers.sp.get_value());
    }
}
