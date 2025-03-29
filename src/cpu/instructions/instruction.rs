use std::sync::Arc;
use parking_lot::Mutex;
use crate::cpu::alu::ALU;
use crate::cpu::instructions::*;
use crate::cpu::registers::Registers;
use crate::memory::MemoryController;

macro_rules! return_if_is_instruction {
    ($instruction:ty, $opcode:expr) => {
        if let Some(instruction) = <$instruction>::from_opcode($opcode) {
            return instruction;
        }
    };
}


pub trait Instruction {

    fn from_opcode(opcode: &u8) -> Option<Box<dyn Instruction>> where Self: Sized;
    fn get_opcode(&self) -> u8;

    fn act(&mut self, registers: &mut Registers, alu: &mut ALU, memory_controller: Arc<Mutex<MemoryController>>, enable_interrupts: &mut bool) -> bool; //returns whether the CPU should return the next instruction

}


pub fn decode_instruction(opcode: &u8) -> Box<dyn Instruction> {
    return_if_is_instruction!(Nop, opcode); //0x00
    return_if_is_instruction!(LdSpNn, opcode); //0x31
    return_if_is_instruction!(JpNn, opcode); //0xC3
    return_if_is_instruction!(Di, opcode); //0xF3

    //if fallen through, return a generic bad instruction
    BadInstruction::from_opcode(opcode).unwrap()
}
