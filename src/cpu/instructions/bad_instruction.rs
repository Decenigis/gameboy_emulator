use std::sync::Arc;
use parking_lot::Mutex;
use crate::cpu::alu::ALU;
use crate::cpu::instructions::instruction::Instruction;
use crate::cpu::registers::Registers;
use crate::memory::MemoryController;

pub struct BadInstruction {
    opcode: u8
}

impl Instruction for BadInstruction {
    fn from_opcode(opcode: &u8) -> Option<Box<dyn Instruction>> {
        Some(Box::new(Self {opcode: *opcode}))
    }

    fn get_opcode(&self) -> u8 {
        self.opcode
    }

    fn act(&mut self, _registers: &mut Registers, _alu: &mut ALU, _memory_controller: Arc<Mutex<MemoryController>>) -> bool {
        false
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_opcode_returns_always() {
        let instruction1 = BadInstruction::from_opcode(&0x00);
        let instruction2 = BadInstruction::from_opcode(&0xDD);

        assert_eq!(true, instruction1.is_some());
        assert_eq!(true, instruction2.is_some());
    }

    #[test]
    fn get_opcode_returns_given_opcode() {
        let opcode = 0xDD;
        let instruction = BadInstruction::from_opcode(&opcode).unwrap();

        assert_eq!(opcode, instruction.get_opcode());
    }

    #[test]
    fn act_immediately_returns_false() {
        let mut registers = Registers::new(0, 0, 0, 0, 0, 0);
        let mut alu = ALU::new(registers.f.clone());

        let mut instruction = BadInstruction { opcode: 0x00 };

        let result = instruction.act(&mut registers, &mut alu, Arc::new(Mutex::new(MemoryController::new())));

        assert_eq!(false, result);
    }
}
