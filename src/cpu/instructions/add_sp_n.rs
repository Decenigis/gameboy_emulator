use crate::cpu::alu::ALU;
use crate::cpu::instructions::Instruction;
use crate::cpu::register::Register;
use crate::cpu::registers::Registers;
use crate::memory::{MemoryController, MemoryTrait};
use parking_lot::Mutex;
use std::sync::Arc;
use crate::cpu::register16::Register16;

pub struct AddSpN {
    counter: u8,
    relative_address: u8
}

impl Instruction for AddSpN {

    #[inline]
    fn from_opcode(opcode: &u8) -> Option<Box<dyn Instruction>> {
        if *opcode == 0xE8 {
            return Some(Box::new(AddSpN { counter: 3, relative_address: 0 }))
        }
        None
    }

    fn get_opcode(&self) -> u8 {
        0xE8
    }

    fn act(&mut self, registers: &mut Registers, alu: &mut ALU, memory_controller: Arc<Mutex<MemoryController>>, _enable_interrupts: &mut bool, _is_halted: &mut bool) -> bool {
        if self.counter == 3 {
            self.relative_address = memory_controller.lock().get(registers.pc.get_value());
            registers.pc.increment();
        }
        if self.counter == 2 {
            if self.relative_address & 0x80 != 0 {
                alu.sub(&mut registers.sp, &Register16::new(0x100 - (self.relative_address as u16)));
            }
            else {
                alu.add(&mut registers.sp, &Register16::new(self.relative_address as u16));
            }

            registers.f.borrow_mut().set_bit(ALU::ZERO_FLAG, false);
            registers.f.borrow_mut().set_bit(ALU::SUB_FLAG, false);
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

    reusable_testing_macro!(0xE8, AddSpN);

    #[test]
    fn load_relative_address_on_tick_3() {
        let mut registers = Registers::new(0, 0, 0, 0, 0xC001, 0);
        let mut alu = ALU::new(registers.f.clone());
        let memory = Arc::new(Mutex::new(MemoryController::new()));

        memory.lock().set(0xC001, 0xFE);

        let mut instruction = AddSpN { counter: 3, relative_address: 0 };

        let result = instruction.act(&mut registers, &mut alu, memory.clone(),&mut false, &mut false);

        assert_eq!(false, result);
        assert_eq!(0xFE, instruction.relative_address);
        assert_eq!(0xC002, registers.pc.get_value());
    }

    #[test]
    fn calculate_address_on_tick_2_for_add() {
        let mut registers = Registers::new(0, 0, 0, 0, 0xC001, 0x1000);
        let mut alu = ALU::new(registers.f.clone());
        let memory = Arc::new(Mutex::new(MemoryController::new()));

        let mut instruction = AddSpN { counter: 2, relative_address: 0x01 };

        let result = instruction.act(&mut registers, &mut alu, memory.clone(),&mut false, &mut false);

        assert_eq!(false, result);
        assert_eq!(0x1001, registers.sp.get_value());
        assert_eq!(false, registers.f.borrow().get_bit(ALU::ZERO_FLAG));
        assert_eq!(false, registers.f.borrow().get_bit(ALU::SUB_FLAG));
    }

    #[test]
    fn calculate_address_on_tick_1_for_sub() {
        let mut registers = Registers::new(0, 0, 0, 0, 0xC001, 0x1000);
        let mut alu = ALU::new(registers.f.clone());
        let memory = Arc::new(Mutex::new(MemoryController::new()));

        let mut instruction = AddSpN { counter: 2, relative_address: 0xFF };

        let result = instruction.act(&mut registers, &mut alu, memory.clone(),&mut false, &mut false);

        assert_eq!(false, result);
        assert_eq!(0xFFF, registers.sp.get_value());
        assert_eq!(false, registers.f.borrow().get_bit(ALU::ZERO_FLAG));
        assert_eq!(false, registers.f.borrow().get_bit(ALU::SUB_FLAG));
    }

    #[test]
    fn get_next_instruction_on_tick_0() {
        let mut registers = Registers::new(0, 0, 0, 0, 0xC000, 0);
        let mut alu = ALU::new(registers.f.clone());
        let memory = Arc::new(Mutex::new(MemoryController::new()));

        let mut instruction = AddSpN { counter: 0, relative_address: 0 };

        let result = instruction.act(&mut registers, &mut alu, memory.clone(),&mut false, &mut false);

        assert_eq!(true, result);
    }
}
