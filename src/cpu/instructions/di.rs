use std::sync::Arc;
use parking_lot::Mutex;
use crate::cpu::alu::ALU;
use crate::cpu::instructions::Instruction;
use crate::cpu::registers::Registers;
use crate::memory::MemoryController;

pub struct Di {}

impl Instruction for Di {

    fn from_opcode(opcode: &u8) -> Option<Box<dyn Instruction>> {
        if *opcode == 0xF3 {
            return Some(Box::new(Di {}))
        }
        None
    }

    fn get_opcode(&self) -> u8 {
        0xF3
    }

    fn act(&mut self, _registers: &mut Registers, _alu: &mut ALU, _memory_controller: Arc<Mutex<MemoryController>>, enable_interrupts: &mut bool) -> bool {
        *enable_interrupts = false;

        true
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_opcode_returns_given_0xf3() {
        let instruction = Di::from_opcode(&0xF3);

        assert_eq!(true, instruction.is_some());
    }

    #[test]
    fn from_opcode_returns_none_given_non_0xf3() {
        let instruction = Di::from_opcode(&0x00);

        assert_eq!(true, instruction.is_none());
    }

    #[test]
    fn get_opcode_returns_0xf3() {
        let instruction = Di {};

        assert_eq!(0xF3, instruction.get_opcode());
    }

    #[test]
    fn act_immediately_returns_true() {
        let mut registers = Registers::new(0, 0, 0, 0, 0, 0);
        let mut alu = ALU::new(registers.f.clone());

        let mut instruction = Di {};

        let result = instruction.act(&mut registers, &mut alu, Arc::new(Mutex::new(MemoryController::new())) ,&mut false);

        assert_eq!(true, result);
    }

    #[test]
    fn act_disables_interrupts() {
        let mut registers = Registers::new(0, 0, 0, 0, 0, 0);
        let mut alu = ALU::new(registers.f.clone());

        let mut instruction = Di {};
        let mut enable_interrupts = true;

        instruction.act(&mut registers, &mut alu, Arc::new(Mutex::new(MemoryController::new())), &mut enable_interrupts);

        assert_eq!(false, enable_interrupts);
    }
}
