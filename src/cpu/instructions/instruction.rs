use crate::cpu::alu::ALU;
use crate::cpu::instructions::*;
use crate::cpu::registers::Registers;
use crate::memory::MemoryController;
use crate::{adc_a_r_decode_instruction, add_a_r_decode_instruction, add_hl_rr_decode_instruction, and_a_r_decode_instruction, call_cc_nn_decode_instruction, cp_a_r_decode_instruction, dec_r_decode_instruction, dec_rr_decode_instruction, inc_r_decode_instruction, inc_rr_decode_instruction, jp_cc_nn_decode_instruction, jr_cc_n_decode_instruction, ld_a_rr_decode_instruction, ld_hl_r_decode_instruction, ld_r_hl_decode_instruction, ld_r_n_decode_instruction, ld_r_r_decode_instruction, ld_rr_a_decode_instruction, ld_rr_nn_decode_instruction, or_a_r_decode_instruction, pop_rr_decode_instruction, push_rr_decode_instruction, ret_with_condition_decode_instruction, rst_nn_decode_instruction, sbc_a_r_decode_instruction, sub_a_r_decode_instruction, xor_a_r_decode_instruction};
use parking_lot::Mutex;
use std::sync::Arc;


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

    fn act(&mut self, registers: &mut Registers, alu: &mut ALU, memory_controller: Arc<Mutex<MemoryController>>, enable_interrupts: &mut bool, is_halted: &mut bool) -> bool; //returns whether the CPU should return the next instruction

}


pub fn decode_instruction(opcode: &u8) -> Box<dyn Instruction> {
    return_if_is_instruction!(Nop, opcode);     //0x00

    ld_r_n_decode_instruction!(opcode);
    ld_r_r_decode_instruction!(opcode);
    ld_r_hl_decode_instruction!(opcode);
    ld_hl_r_decode_instruction!(opcode);
    ret_with_condition_decode_instruction!(opcode);
    inc_r_decode_instruction!(opcode);
    inc_rr_decode_instruction!(opcode);
    dec_r_decode_instruction!(opcode);
    dec_rr_decode_instruction!(opcode);
    push_rr_decode_instruction!(opcode);
    pop_rr_decode_instruction!(opcode);
    or_a_r_decode_instruction!(opcode);
    add_a_r_decode_instruction!(opcode);
    adc_a_r_decode_instruction!(opcode);
    and_a_r_decode_instruction!(opcode);
    add_hl_rr_decode_instruction!(opcode);
    sub_a_r_decode_instruction!(opcode);
    sbc_a_r_decode_instruction!(opcode);
    xor_a_r_decode_instruction!(opcode);
    cp_a_r_decode_instruction!(opcode);
    ld_rr_nn_decode_instruction!(opcode);
    ld_rr_a_decode_instruction!(opcode);
    ld_a_rr_decode_instruction!(opcode);
    call_cc_nn_decode_instruction!(opcode);
    jr_cc_n_decode_instruction!(opcode);
    jp_cc_nn_decode_instruction!(opcode);
    rst_nn_decode_instruction!(opcode);

    return_if_is_instruction!(RlcA, opcode);    //0x07
    return_if_is_instruction!(LdNnSp, opcode);  //0x08
    return_if_is_instruction!(RrcA, opcode);    //0x0F
    return_if_is_instruction!(RlA, opcode);     //0x17
    return_if_is_instruction!(JrN, opcode);     //0x18
    return_if_is_instruction!(LdiHlA, opcode);  //0x22
    return_if_is_instruction!(IncHl, opcode);   //0x23
    return_if_is_instruction!(LdiAHl, opcode);  //0x2A
    return_if_is_instruction!(CplA, opcode);    //0x2F
    return_if_is_instruction!(LdSpNn, opcode);  //0x31
    return_if_is_instruction!(LddHlA, opcode);  //0x32
    return_if_is_instruction!(IncHlAddr, opcode);//0x34
    return_if_is_instruction!(DecHlAddr, opcode);//0x35
    return_if_is_instruction!(Scf, opcode);   //0x36
    return_if_is_instruction!(LdHlN, opcode);   //0x37
    return_if_is_instruction!(LddAHl, opcode);  //0x3A
    return_if_is_instruction!(Halt, opcode);    //0x76
    return_if_is_instruction!(AddAHl, opcode);  //0x86
    return_if_is_instruction!(AddAA, opcode);   //0x87
    return_if_is_instruction!(AndAHl, opcode);  //0xA6
    return_if_is_instruction!(AndAA, opcode);   //0xA7
    return_if_is_instruction!(XorA, opcode);    //0xAF
    return_if_is_instruction!(JpNn, opcode);    //0xC3
    return_if_is_instruction!(AddAN, opcode);   //0xC6
    return_if_is_instruction!(Ret, opcode);     //0xC9
    return_if_is_instruction!(Bitwise, opcode); //0xCB
    return_if_is_instruction!(CallNn, opcode);  //0xCD
    return_if_is_instruction!(AdcAN, opcode);   //0xCE
    return_if_is_instruction!(SubAN, opcode);   //0xD6
    return_if_is_instruction!(Reti, opcode);    //0xD9
    return_if_is_instruction!(SbcAN, opcode);   //0xDE
    return_if_is_instruction!(LdhNA, opcode);   //0xE0
    return_if_is_instruction!(LdhCA, opcode);   //0xE2
    return_if_is_instruction!(AndAN, opcode);   //0xE6
    return_if_is_instruction!(JpHl, opcode);    //0xE9
    return_if_is_instruction!(LdNNA, opcode);   //0xEA
    return_if_is_instruction!(LdhAN, opcode);   //0xF0
    return_if_is_instruction!(Di, opcode);      //0xF3
    return_if_is_instruction!(LdHlSpN, opcode); //0xF8
    return_if_is_instruction!(LdSpHl, opcode);  //0xF9
    return_if_is_instruction!(LdANN, opcode);   //0xFA
    return_if_is_instruction!(Ei, opcode);      //0xFB
    return_if_is_instruction!(CpAN, opcode);    //0xFE

    //if fallen through, return a generic bad instruction
    BadInstruction::from_opcode(opcode).unwrap()
}
