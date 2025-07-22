use crate::cpu::alu::ALU;
use crate::cpu::instructions::Instruction;
use crate::cpu::register::Register;
use crate::cpu::registers::Registers;
use crate::memory::MemoryController;
use parking_lot::Mutex;
use std::sync::Arc;

pub struct JpHl {}

impl Instruction for JpHl {

    #[inline]
    fn from_opcode(opcode: &u8) -> Option<Box<dyn Instruction>> {
        if *opcode == 0xE9 {
            return Some(Box::new(JpHl {  }))
        }
        None
    }

    fn get_opcode(&self) -> u8 {
        0xE9
    }

    fn act(&mut self, registers: &mut Registers, _alu: &mut ALU, memory_controller: Arc<Mutex<MemoryController>>, _enable_interrupts: &mut bool, _is_halted: &mut bool) -> bool {
        registers.pc.set_value(registers.hl.get_value());

        true
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    reusable_testing_macro!(0xE9, JpHl);

    #[test]
    fn jump_and_get_next_instruction_immediately() {
        let mut registers = Registers::new(0, 0, 0, 0xD000, 0xC000, 0);
        let mut alu = ALU::new(registers.f.clone());
        let memory = Arc::new(Mutex::new(MemoryController::new()));

        let mut instruction = JpHl { };

        let result = instruction.act(&mut registers, &mut alu, memory.clone(),&mut false, &mut false);

        assert_eq!(true, result);
        assert_eq!(0xD000, registers.pc.get_value());
    }
}
