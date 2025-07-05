use std::sync::Arc;
use parking_lot::Mutex;
use crate::cpu::alu::ALU;
use crate::cpu::instructions::*;
use crate::cpu::registers::Registers;
use crate::{add_a_r_decode_instruction, dec_r_decode_instruction, inc_r_decode_instruction, ld_r_n_decode_instruction, ld_r_r_decode_instruction, ld_rr_nn_decode_instruction, pop_rr_decode_instruction, push_rr_decode_instruction, ret_with_condition_decode_instruction};
use crate::memory::MemoryController;


/**
This macro should be used where instructions use a reusable handler,
such as ld_r_n
*/
#[macro_export] macro_rules! reusable_testing_macro {
    ($opcode:literal, $instruction:ty) => {
        #[test]
        fn from_opcode_returns_given_right_opcode() {
            let instruction = <$instruction>::from_opcode(&$opcode);

            assert_eq!(true, instruction.is_some());
        }

        #[test]
        fn from_opcode_returns_none_given_wrong_opcode() {
            let instruction = <$instruction>::from_opcode(&0x00);

            assert_eq!(true, instruction.is_none());
        }

        #[test]
        fn get_opcode_returns_opcode() {
            let instruction = <$instruction>::from_opcode(&$opcode).unwrap();

            assert_eq!($opcode, instruction.get_opcode());
        }
    };
}


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

    ld_r_n_decode_instruction!(opcode);
    ld_r_r_decode_instruction!(opcode);
    ret_with_condition_decode_instruction!(opcode);
    inc_r_decode_instruction!(opcode);
    dec_r_decode_instruction!(opcode);
    push_rr_decode_instruction!(opcode);
    pop_rr_decode_instruction!(opcode);
    add_a_r_decode_instruction!(opcode);
    ld_rr_nn_decode_instruction!(opcode);

    return_if_is_instruction!(LdBcNn, opcode);  //0x01
    return_if_is_instruction!(DecBc, opcode);   //0x0B
    return_if_is_instruction!(JrNz, opcode);    //0x20
    return_if_is_instruction!(LdHlNn, opcode);  //0x21
    return_if_is_instruction!(IncHl, opcode);   //0x23
    return_if_is_instruction!(LdiAHl, opcode);   //0x23
    return_if_is_instruction!(LdSpNn, opcode);  //0x31
    return_if_is_instruction!(LdHlN, opcode);   //0x36
    return_if_is_instruction!(AddAA, opcode);   //0x87
    return_if_is_instruction!(XorA, opcode);    //0xAF
    return_if_is_instruction!(OrB, opcode);     //0xB0
    return_if_is_instruction!(JpNn, opcode);    //0xC3
    return_if_is_instruction!(Ret, opcode);     //0xC9
    return_if_is_instruction!(CallNn, opcode);  //0xCD
    return_if_is_instruction!(AddAN, opcode);   //0xD6
    return_if_is_instruction!(SubAN, opcode);   //0xD6
    return_if_is_instruction!(LdhNA, opcode);   //0xE0
    return_if_is_instruction!(LdhCA, opcode);   //0xE2
    return_if_is_instruction!(LdNNA, opcode);   //0xEA
    return_if_is_instruction!(LdhAN, opcode);   //0xF0
    return_if_is_instruction!(Di, opcode);      //0xF3
    return_if_is_instruction!(LdANN, opcode);   //0xFA
    return_if_is_instruction!(CpAN, opcode);    //0xFE

    //if fallen through, return a generic bad instruction
    BadInstruction::from_opcode(opcode).unwrap()
}
