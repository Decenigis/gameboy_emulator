use crate::cpu::alu::ALU;
use crate::cpu::instructions::Instruction;
use crate::cpu::register::Register;
use crate::cpu::registers::Registers;
use crate::memory::MemoryController;
use parking_lot::Mutex;
use std::sync::Arc;

//Timings for this one might be a bit off due to te 4-cycle alignment

pub struct LdAC {}

impl Instruction for LdAC {

    #[inline]
    fn from_opcode(opcode: &u8) -> Option<Box<dyn Instruction>> {
        if *opcode == 0x79 {
            return Some(Box::new(LdAC {}))
        }
        None
    }

    fn get_opcode(&self) -> u8 {
        0x79
    }

    fn act(&mut self, registers: &mut Registers, _alu: &mut ALU, _memory_controller: Arc<Mutex<MemoryController>>, _enable_interrupts: &mut bool) -> bool {
        registers.a.borrow_mut().set_value(registers.c.borrow().get_value());

        true
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_opcode_returns_given_0x79() {
        let instruction = LdAC::from_opcode(&0x79);

        assert_eq!(true, instruction.is_some());
    }

    #[test]
    fn from_opcode_returns_none_given_non_0x79() {
        let instruction = LdAC::from_opcode(&0x00);

        assert_eq!(true, instruction.is_none());
    }

    #[test]
    fn get_opcode_returns_0x79() {
        let instruction = LdAC {};

        assert_eq!(0x79, instruction.get_opcode());
    }

    #[test]
    fn move_c_to_a_and_get_next_instruction_immediately() {
        let mut registers = Registers::new(0, 0, 0, 0, 0xC000, 0);
        let mut alu = ALU::new(registers.f.clone());
        let memory = Arc::new(Mutex::new(MemoryController::new()));

        let mut instruction = LdAC {};

        let result = instruction.act(&mut registers, &mut alu, memory.clone(),&mut false);

        assert_eq!(true, result);
    }
}
