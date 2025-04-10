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

    #[allow(dead_code)]
    fn get_opcode(&self) -> u8;

    fn act(&mut self, registers: &mut Registers, alu: &mut ALU, memory_controller: Arc<Mutex<MemoryController>>, enable_interrupts: &mut bool) -> bool; //returns whether the CPU should return the next instruction

}


pub fn decode_instruction(opcode: &u8) -> Box<dyn Instruction> {
    return_if_is_instruction!(Nop, opcode);     //0x00
    return_if_is_instruction!(LdBcNn, opcode);  //0x01
    return_if_is_instruction!(DecBc, opcode);   //0x0B
    return_if_is_instruction!(JrNz, opcode);    //0x20
    return_if_is_instruction!(LdHlNn, opcode);  //0x21
    return_if_is_instruction!(IncHl, opcode);   //0x23
    return_if_is_instruction!(LdSpNn, opcode);  //0x31
    return_if_is_instruction!(LdHlN, opcode);   //0x36
    return_if_is_instruction!(LdAN, opcode);    //0x3E
    return_if_is_instruction!(LdAC, opcode);    //0x79
    return_if_is_instruction!(XorA, opcode);    //0xAF
    return_if_is_instruction!(OrB, opcode);     //0xB0
    return_if_is_instruction!(JpNn, opcode);    //0xC3
    return_if_is_instruction!(Ret, opcode);     //0xC9
    return_if_is_instruction!(CallNn, opcode);  //0xCD
    return_if_is_instruction!(LdhNA, opcode);   //0xE0
    return_if_is_instruction!(Di, opcode);      //0xF3

    //if fallen through, return a generic bad instruction
    BadInstruction::from_opcode(opcode).unwrap()
}
