use crate::cpu::alu::ALU;
use crate::cpu::instructions::Instruction;
use crate::cpu::register::Register;
use crate::cpu::registers::Registers;
use crate::memory::{MemoryController, MemoryTrait};
use parking_lot::Mutex;
use paste::paste;
use std::sync::Arc;


macro_rules! ld_r_r {
    ($opcode:literal, $reg_1:ident, $reg_2:ident, $reg_1_upper:ident, $reg_2_upper:ident) => {
        paste!{
            pub struct [<Ld $reg_1_upper $reg_2_upper>] {}

            impl Instruction for [<Ld $reg_1_upper $reg_2_upper>] {
                #[inline]
                fn from_opcode(opcode: &u8) -> Option<Box<dyn Instruction>> {
                    if *opcode == $opcode {
                        return Some(Box::new([<Ld $reg_1_upper $reg_2_upper>]{}))
                    }
                    None
                }

                fn get_opcode(&self) -> u8 {
                    $opcode
                }

                fn act(&mut self, registers: &mut Registers, _alu: &mut ALU, _memory_controller: Arc<Mutex<MemoryController>>, _enable_interrupts: &mut bool, _is_halted: &mut bool) -> bool {
                    let value = registers.$reg_2.borrow().get_value();
                    registers.$reg_1.borrow_mut().set_value(value);
                    true
                }
            }


            #[cfg(test)]
            mod [<ld_ $reg_1 _ $reg_2 _tests>] {
                use super::*;

                reusable_testing_macro!($opcode, [<Ld $reg_1_upper $reg_2_upper>]);

                #[test]
                fn load_r2_into_r1_and_get_next_instruction() {
                    let mut registers = Registers::new(0, 0, 0, 0, 0xC000, 0);

                    registers.$reg_2.borrow_mut().set_value(0x12);

                    let memory = Arc::new(Mutex::new(MemoryController::new()));
                    let mut alu = ALU::new(registers.f.clone());

                    let mut instruction = [<Ld $reg_1_upper $reg_2_upper>] {};

                    let result = instruction.act(&mut registers, &mut alu, memory.clone(), &mut false, &mut false);

                    assert_eq!(true, result);
                    assert_eq!(registers.$reg_1.borrow().get_value(), registers.$reg_1.borrow().get_value());
                }
            }
        }
    }
}

ld_r_r!(0x78, a, b, A, B);
ld_r_r!(0x79, a, c, A, C);
ld_r_r!(0x7A, a, d, A, D);
ld_r_r!(0x7B, a, e, A, E);
ld_r_r!(0x7C, a, h, A, H);
ld_r_r!(0x7D, a, l, A, L);
ld_r_r!(0x7F, a, a, A, A);
ld_r_r!(0x40, b, b, B, B);
ld_r_r!(0x41, b, c, B, C);
ld_r_r!(0x42, b, d, B, D);
ld_r_r!(0x43, b, e, B, E);
ld_r_r!(0x44, b, h, B, H);
ld_r_r!(0x45, b, l, B, L);
ld_r_r!(0x47, b, a, B, A);
ld_r_r!(0x48, c, b, C, B);
ld_r_r!(0x49, c, c, C, C);
ld_r_r!(0x4A, c, d, C, D);
ld_r_r!(0x4B, c, e, C, E);
ld_r_r!(0x4C, c, h, C, H);
ld_r_r!(0x4D, c, l, C, L);
ld_r_r!(0x4F, c, a, C, A);
ld_r_r!(0x50, d, b, D, B);
ld_r_r!(0x51, d, c, D, C);
ld_r_r!(0x52, d, d, D, D);
ld_r_r!(0x53, d, e, D, E);
ld_r_r!(0x54, d, h, D, H);
ld_r_r!(0x55, d, l, D, L);
ld_r_r!(0x57, d, a, D, A);
ld_r_r!(0x58, e, b, E, B);
ld_r_r!(0x59, e, c, E, C);
ld_r_r!(0x5A, e, d, E, D);
ld_r_r!(0x5B, e, e, E, E);
ld_r_r!(0x5C, e, h, E, H);
ld_r_r!(0x5D, e, l, E, L);
ld_r_r!(0x5F, e, a, E, A);
ld_r_r!(0x60, h, b, H, B);
ld_r_r!(0x61, h, c, H, C);
ld_r_r!(0x62, h, d, H, D);
ld_r_r!(0x63, h, e, H, E);
ld_r_r!(0x64, h, h, H, H);
ld_r_r!(0x65, h, l, H, L);
ld_r_r!(0x67, h, a, H, A);
ld_r_r!(0x68, l, b, L, B);
ld_r_r!(0x69, l, c, L, C);
ld_r_r!(0x6A, l, d, L, D);
ld_r_r!(0x6B, l, e, L, E);
ld_r_r!(0x6C, l, h, L, H);
ld_r_r!(0x6D, l, l, L, L);
ld_r_r!(0x6F, l, a, L, A);


#[macro_export] macro_rules! ld_r_r_decode_instruction {
    //this really sucks
    ($opcode:expr) => {
        return_if_is_instruction!(LdAB, $opcode);
        return_if_is_instruction!(LdAC, $opcode);
        return_if_is_instruction!(LdAD, $opcode);
        return_if_is_instruction!(LdAE, $opcode);
        return_if_is_instruction!(LdAH, $opcode);
        return_if_is_instruction!(LdAL, $opcode);
        return_if_is_instruction!(LdAA, $opcode);
        return_if_is_instruction!(LdBB, $opcode);
        return_if_is_instruction!(LdBC, $opcode);
        return_if_is_instruction!(LdBD, $opcode);
        return_if_is_instruction!(LdBE, $opcode);
        return_if_is_instruction!(LdBH, $opcode);
        return_if_is_instruction!(LdBL, $opcode);
        return_if_is_instruction!(LdBA, $opcode);
        return_if_is_instruction!(LdCB, $opcode);
        return_if_is_instruction!(LdCC, $opcode);
        return_if_is_instruction!(LdCD, $opcode);
        return_if_is_instruction!(LdCE, $opcode);
        return_if_is_instruction!(LdCH, $opcode);
        return_if_is_instruction!(LdCL, $opcode);
        return_if_is_instruction!(LdCA, $opcode);
        return_if_is_instruction!(LdDB, $opcode);
        return_if_is_instruction!(LdDC, $opcode);
        return_if_is_instruction!(LdDD, $opcode);
        return_if_is_instruction!(LdDE, $opcode);
        return_if_is_instruction!(LdDH, $opcode);
        return_if_is_instruction!(LdDL, $opcode);
        return_if_is_instruction!(LdDA, $opcode);
        return_if_is_instruction!(LdEB, $opcode);
        return_if_is_instruction!(LdEC, $opcode);
        return_if_is_instruction!(LdED, $opcode);
        return_if_is_instruction!(LdEE, $opcode);
        return_if_is_instruction!(LdEH, $opcode);
        return_if_is_instruction!(LdEL, $opcode);
        return_if_is_instruction!(LdEA, $opcode);
        return_if_is_instruction!(LdHB, $opcode);
        return_if_is_instruction!(LdHC, $opcode);
        return_if_is_instruction!(LdHD, $opcode);
        return_if_is_instruction!(LdHE, $opcode);
        return_if_is_instruction!(LdHH, $opcode);
        return_if_is_instruction!(LdHL, $opcode);
        return_if_is_instruction!(LdHA, $opcode);
        return_if_is_instruction!(LdLB, $opcode);
        return_if_is_instruction!(LdLC, $opcode);
        return_if_is_instruction!(LdLD, $opcode);
        return_if_is_instruction!(LdLE, $opcode);
        return_if_is_instruction!(LdLH, $opcode);
        return_if_is_instruction!(LdLL, $opcode);
        return_if_is_instruction!(LdLA, $opcode);
    }
}
