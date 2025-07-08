use crate::cpu::alu::ALU;
use crate::cpu::instructions::Instruction;
use crate::cpu::register::Register;
use crate::cpu::registers::Registers;
use crate::memory::{MemoryController, MemoryTrait};
use parking_lot::Mutex;
use std::sync::Arc;

pub struct JrN {
    counter: u8,
    address: u16
}

impl Instruction for JrN {

    #[inline]
    fn from_opcode(opcode: &u8) -> Option<Box<dyn Instruction>> {
        if *opcode == 0x18 {
            return Some(Box::new(JrN { counter: 2, address: 0 }))
        }
        None
    }

    fn get_opcode(&self) -> u8 {
        0x18
    }

    fn act(&mut self, registers: &mut Registers, _alu: &mut ALU, memory_controller: Arc<Mutex<MemoryController>>, _enable_interrupts: &mut bool, _is_halted: &mut bool) -> bool {
        if self.counter == 1 {
            let relative_address = memory_controller.lock().get(registers.pc.get_value());
            registers.pc.increment();

            if relative_address & 0x80 != 0 {
                self.address  = registers.pc.get_value().wrapping_sub(0x100 - (relative_address as u16));
            }
            else {
                self.address = registers.pc.get_value().wrapping_add(relative_address as u16);
            }
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

    reusable_testing_macro!(0x18, JrN);

    #[test]
    fn calculate_address_on_tick_1_for_sub() {
        let mut registers = Registers::new(0, 0, 0, 0, 0xC001, 0);
        let mut alu = ALU::new(registers.f.clone());
        let memory = Arc::new(Mutex::new(MemoryController::new()));

        memory.lock().set(0xC001, 0xFE);

        let mut instruction = JrN { counter: 1, address: 0 };

        let result = instruction.act(&mut registers, &mut alu, memory.clone(),&mut false, &mut false);

        assert_eq!(false, result);

        assert_eq!(0xC000, instruction.address);
    }

    #[test]
    fn calculate_address_on_tick_1_for_add() {
        let mut registers = Registers::new(0, 0, 0, 0, 0xC001, 0);
        let mut alu = ALU::new(registers.f.clone());
        let memory = Arc::new(Mutex::new(MemoryController::new()));

        memory.lock().set(0xC001, 0x02);

        let mut instruction = JrN { counter: 1, address: 0 };

        let result = instruction.act(&mut registers, &mut alu, memory.clone(),&mut false, &mut false);

        assert_eq!(false, result);
        assert_eq!(0xC004, instruction.address);
    }

    #[test]
    fn update_pc_on_tick_0_and_get_next_instruction() {
        let mut registers = Registers::new(0, 0, 0, 0, 0xC000, 0);
        let mut alu = ALU::new(registers.f.clone());
        let memory = Arc::new(Mutex::new(MemoryController::new()));

        let mut instruction = JrN { counter: 0, address: 0x1234 };

        let result = instruction.act(&mut registers, &mut alu, memory.clone(),&mut false, &mut false);

        assert_eq!(true, result);

        assert_eq!(0x1234, registers.pc.get_value());
    }
}
