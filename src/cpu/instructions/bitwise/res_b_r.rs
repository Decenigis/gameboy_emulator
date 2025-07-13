use crate::cpu::alu::ALU;
use crate::cpu::instructions::Instruction;
use crate::cpu::register::Register;
use crate::cpu::registers::Registers;
use crate::memory::MemoryController;
use parking_lot::Mutex;
use paste::paste;
use std::sync::Arc;


macro_rules! res_b_r {
    ($opcode:literal, $bit:literal, $reg:ident, $reg_upper:ident) => {
        paste!{
            pub struct [<Res $bit $reg_upper>] {
                counter: u8
            }

            impl Instruction for [<Res $bit $reg_upper>] {
                #[inline]
                fn from_opcode(opcode: &u8) -> Option<Box<dyn Instruction>> {
                    if *opcode == $opcode {
                        return Some(Box::new([<Res $bit $reg_upper>]{ counter: 1 }))
                    }
                    None
                }

                fn get_opcode(&self) -> u8 {
                    $opcode
                }

                fn act(&mut self, registers: &mut Registers, _alu: &mut ALU, _memory_controller: Arc<Mutex<MemoryController>>, _enable_interrupts: &mut bool, _is_halted: &mut bool) -> bool {
                    if self.counter == 1 {
                        registers.$reg.borrow_mut().set_bit($bit, false);
                    }
                    else if self.counter == 0 {
                        return true;
                    }

                    self.counter -= 1;
                    false
                }
            }


            #[cfg(test)]
            mod [<res_ $bit _ $reg _tests>] {
                use super::*;

                reusable_testing_macro!($opcode, [<Res $bit $reg_upper>]);

                #[test]
                fn tests_bit_when_set_on_tick_1() {
                    let mut registers = Registers::new(0, 0, 0, 0, 0xC000, 0);
                    let memory = Arc::new(Mutex::new(MemoryController::new()));
                    let mut alu = ALU::new(registers.f.clone());

                    registers.$reg.borrow_mut().set_bit($bit, true);

                    let mut instruction = [<Res $bit $reg_upper>] { counter: 1};

                    let result = instruction.act(&mut registers, &mut alu, memory.clone(), &mut false, &mut false);

                    assert_eq!(false, result);
                    assert_eq!(false, registers.$reg.borrow_mut().get_bit($bit));
                }

                #[test]
                fn gets_next_instruction_on_tick_0() {
                    let mut registers = Registers::new(0, 0, 0, 0, 0xC000, 0);
                    let memory = Arc::new(Mutex::new(MemoryController::new()));
                    let mut alu = ALU::new(registers.f.clone());

                    let mut instruction = [<Res $bit $reg_upper>] { counter: 0};

                    let result = instruction.act(&mut registers, &mut alu, memory.clone(), &mut false, &mut false);

                    assert_eq!(true, result);
                }
            }
        }
    }
}

res_b_r!(0x80, 0, b, B);
res_b_r!(0x81, 0, c, C);
res_b_r!(0x82, 0, d, D);
res_b_r!(0x83, 0, e, E);
res_b_r!(0x84, 0, h, H);
res_b_r!(0x85, 0, l, L);
res_b_r!(0x87, 0, a, A);
res_b_r!(0x88, 1, b, B);
res_b_r!(0x89, 1, c, C);
res_b_r!(0x8A, 1, d, D);
res_b_r!(0x8B, 1, e, E);
res_b_r!(0x8C, 1, h, H);
res_b_r!(0x8D, 1, l, L);
res_b_r!(0x8F, 1, a, A);
res_b_r!(0x90, 2, b, B);
res_b_r!(0x91, 2, c, C);
res_b_r!(0x92, 2, d, D);
res_b_r!(0x93, 2, e, E);
res_b_r!(0x94, 2, h, H);
res_b_r!(0x95, 2, l, L);
res_b_r!(0x97, 2, a, A);
res_b_r!(0x98, 3, b, B);
res_b_r!(0x99, 3, c, C);
res_b_r!(0x9A, 3, d, D);
res_b_r!(0x9B, 3, e, E);
res_b_r!(0x9C, 3, h, H);
res_b_r!(0x9D, 3, l, L);
res_b_r!(0x9F, 3, a, A);
res_b_r!(0xA0, 4, b, B);
res_b_r!(0xA1, 4, c, C);
res_b_r!(0xA2, 4, d, D);
res_b_r!(0xA3, 4, e, E);
res_b_r!(0xA4, 4, h, H);
res_b_r!(0xA5, 4, l, L);
res_b_r!(0xA7, 4, a, A);
res_b_r!(0xA8, 5, b, B);
res_b_r!(0xA9, 5, c, C);
res_b_r!(0xAA, 5, d, D);
res_b_r!(0xAB, 5, e, E);
res_b_r!(0xAC, 5, h, H);
res_b_r!(0xAD, 5, l, L);
res_b_r!(0xAF, 5, a, A);
res_b_r!(0xB0, 6, b, B);
res_b_r!(0xB1, 6, c, C);
res_b_r!(0xB2, 6, d, D);
res_b_r!(0xB3, 6, e, E);
res_b_r!(0xB4, 6, h, H);
res_b_r!(0xB5, 6, l, L);
res_b_r!(0xB7, 6, a, A);
res_b_r!(0xB8, 7, b, B);
res_b_r!(0xB9, 7, c, C);
res_b_r!(0xBA, 7, d, D);
res_b_r!(0xBB, 7, e, E);
res_b_r!(0xBC, 7, h, H);
res_b_r!(0xBD, 7, l, L);
res_b_r!(0xBF, 7, a, A);



#[macro_export] macro_rules! res_b_r_decode_instruction {
    ($opcode:expr) => {
        return_if_is_instruction!(Res0B, $opcode);
        return_if_is_instruction!(Res0C, $opcode);
        return_if_is_instruction!(Res0D, $opcode);
        return_if_is_instruction!(Res0E, $opcode);
        return_if_is_instruction!(Res0H, $opcode);
        return_if_is_instruction!(Res0L, $opcode);
        return_if_is_instruction!(Res0A, $opcode);
        return_if_is_instruction!(Res1B, $opcode);
        return_if_is_instruction!(Res1C, $opcode);
        return_if_is_instruction!(Res1D, $opcode);
        return_if_is_instruction!(Res1E, $opcode);
        return_if_is_instruction!(Res1H, $opcode);
        return_if_is_instruction!(Res1L, $opcode);
        return_if_is_instruction!(Res1A, $opcode);
        return_if_is_instruction!(Res2B, $opcode);
        return_if_is_instruction!(Res2C, $opcode);
        return_if_is_instruction!(Res2D, $opcode);
        return_if_is_instruction!(Res2E, $opcode);
        return_if_is_instruction!(Res2H, $opcode);
        return_if_is_instruction!(Res2L, $opcode);
        return_if_is_instruction!(Res2A, $opcode);
        return_if_is_instruction!(Res3B, $opcode);
        return_if_is_instruction!(Res3C, $opcode);
        return_if_is_instruction!(Res3D, $opcode);
        return_if_is_instruction!(Res3E, $opcode);
        return_if_is_instruction!(Res3H, $opcode);
        return_if_is_instruction!(Res3L, $opcode);
        return_if_is_instruction!(Res3A, $opcode);
        return_if_is_instruction!(Res4B, $opcode);
        return_if_is_instruction!(Res4C, $opcode);
        return_if_is_instruction!(Res4D, $opcode);
        return_if_is_instruction!(Res4E, $opcode);
        return_if_is_instruction!(Res4H, $opcode);
        return_if_is_instruction!(Res4L, $opcode);
        return_if_is_instruction!(Res4A, $opcode);
        return_if_is_instruction!(Res5B, $opcode);
        return_if_is_instruction!(Res5C, $opcode);
        return_if_is_instruction!(Res5D, $opcode);
        return_if_is_instruction!(Res5E, $opcode);
        return_if_is_instruction!(Res5H, $opcode);
        return_if_is_instruction!(Res5L, $opcode);
        return_if_is_instruction!(Res5A, $opcode);
        return_if_is_instruction!(Res6B, $opcode);
        return_if_is_instruction!(Res6C, $opcode);
        return_if_is_instruction!(Res6D, $opcode);
        return_if_is_instruction!(Res6E, $opcode);
        return_if_is_instruction!(Res6H, $opcode);
        return_if_is_instruction!(Res6L, $opcode);
        return_if_is_instruction!(Res6A, $opcode);
        return_if_is_instruction!(Res7B, $opcode);
        return_if_is_instruction!(Res7C, $opcode);
        return_if_is_instruction!(Res7D, $opcode);
        return_if_is_instruction!(Res7E, $opcode);
        return_if_is_instruction!(Res7H, $opcode);
        return_if_is_instruction!(Res7L, $opcode);
        return_if_is_instruction!(Res7A, $opcode);
    }
}
