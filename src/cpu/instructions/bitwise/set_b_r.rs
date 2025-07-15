use crate::cpu::alu::ALU;
use crate::cpu::instructions::Instruction;
use crate::cpu::register::Register;
use crate::cpu::registers::Registers;
use crate::memory::MemoryController;
use parking_lot::Mutex;
use paste::paste;
use std::sync::Arc;


macro_rules! set_b_r {
    ($opcode:literal, $bit:literal, $reg:ident, $reg_upper:ident) => {
        paste!{
            pub struct [<Set $bit $reg_upper>] {
                counter: u8
            }

            impl Instruction for [<Set $bit $reg_upper>] {
                #[inline]
                fn from_opcode(opcode: &u8) -> Option<Box<dyn Instruction>> {
                    if *opcode == $opcode {
                        return Some(Box::new([<Set $bit $reg_upper>]{ counter: 1 }))
                    }
                    None
                }

                fn get_opcode(&self) -> u8 {
                    $opcode
                }

                fn act(&mut self, registers: &mut Registers, _alu: &mut ALU, _memory_controller: Arc<Mutex<MemoryController>>, _enable_interrupts: &mut bool, _is_halted: &mut bool) -> bool {
                    if self.counter == 1 {
                        registers.$reg.borrow_mut().set_bit($bit, true);
                    }
                    else if self.counter == 0 {
                        return true;
                    }

                    self.counter -= 1;
                    false
                }
            }


            #[cfg(test)]
            mod [<set_ $bit _ $reg _tests>] {
                use super::*;

                reusable_testing_macro!($opcode, [<Set $bit $reg_upper>]);

                #[test]
                fn tests_bit_when_set_on_tick_1() {
                    let mut registers = Registers::new(0, 0, 0, 0, 0xC000, 0);
                    let memory = Arc::new(Mutex::new(MemoryController::new()));
                    let mut alu = ALU::new(registers.f.clone());

                    registers.$reg.borrow_mut().set_bit($bit, false);

                    let mut instruction = [<Set $bit $reg_upper>] { counter: 1};

                    let setult = instruction.act(&mut registers, &mut alu, memory.clone(), &mut false, &mut false);

                    assert_eq!(false, setult);
                    assert_eq!(true, registers.$reg.borrow_mut().get_bit($bit));
                }

                #[test]
                fn gets_next_instruction_on_tick_0() {
                    let mut registers = Registers::new(0, 0, 0, 0, 0xC000, 0);
                    let memory = Arc::new(Mutex::new(MemoryController::new()));
                    let mut alu = ALU::new(registers.f.clone());

                    let mut instruction = [<Set $bit $reg_upper>] { counter: 0};

                    let setult = instruction.act(&mut registers, &mut alu, memory.clone(), &mut false, &mut false);

                    assert_eq!(true, setult);
                }
            }
        }
    }
}

set_b_r!(0xC0, 0, b, B);
set_b_r!(0xC1, 0, c, C);
set_b_r!(0xC2, 0, d, D);
set_b_r!(0xC3, 0, e, E);
set_b_r!(0xC4, 0, h, H);
set_b_r!(0xC5, 0, l, L);
set_b_r!(0xC7, 0, a, A);
set_b_r!(0xC8, 1, b, B);
set_b_r!(0xC9, 1, c, C);
set_b_r!(0xCA, 1, d, D);
set_b_r!(0xCB, 1, e, E);
set_b_r!(0xCC, 1, h, H);
set_b_r!(0xCD, 1, l, L);
set_b_r!(0xCF, 1, a, A);
set_b_r!(0xD0, 2, b, B);
set_b_r!(0xD1, 2, c, C);
set_b_r!(0xD2, 2, d, D);
set_b_r!(0xD3, 2, e, E);
set_b_r!(0xD4, 2, h, H);
set_b_r!(0xD5, 2, l, L);
set_b_r!(0xD7, 2, a, A);
set_b_r!(0xD8, 3, b, B);
set_b_r!(0xD9, 3, c, C);
set_b_r!(0xDA, 3, d, D);
set_b_r!(0xDB, 3, e, E);
set_b_r!(0xDC, 3, h, H);
set_b_r!(0xDD, 3, l, L);
set_b_r!(0xDF, 3, a, A);
set_b_r!(0xE0, 4, b, B);
set_b_r!(0xE1, 4, c, C);
set_b_r!(0xE2, 4, d, D);
set_b_r!(0xE3, 4, e, E);
set_b_r!(0xE4, 4, h, H);
set_b_r!(0xE5, 4, l, L);
set_b_r!(0xE7, 4, a, A);
set_b_r!(0xE8, 5, b, B);
set_b_r!(0xE9, 5, c, C);
set_b_r!(0xEA, 5, d, D);
set_b_r!(0xEB, 5, e, E);
set_b_r!(0xEC, 5, h, H);
set_b_r!(0xED, 5, l, L);
set_b_r!(0xEF, 5, a, A);
set_b_r!(0xF0, 6, b, B);
set_b_r!(0xF1, 6, c, C);
set_b_r!(0xF2, 6, d, D);
set_b_r!(0xF3, 6, e, E);
set_b_r!(0xF4, 6, h, H);
set_b_r!(0xF5, 6, l, L);
set_b_r!(0xF7, 6, a, A);
set_b_r!(0xF8, 7, b, B);
set_b_r!(0xF9, 7, c, C);
set_b_r!(0xFA, 7, d, D);
set_b_r!(0xFB, 7, e, E);
set_b_r!(0xFC, 7, h, H);
set_b_r!(0xFD, 7, l, L);
set_b_r!(0xFF, 7, a, A);



#[macro_export] macro_rules! set_b_r_decode_instruction {
    ($opcode:expr) => {
        return_if_is_instruction!(Set0B, $opcode);
        return_if_is_instruction!(Set0C, $opcode);
        return_if_is_instruction!(Set0D, $opcode);
        return_if_is_instruction!(Set0E, $opcode);
        return_if_is_instruction!(Set0H, $opcode);
        return_if_is_instruction!(Set0L, $opcode);
        return_if_is_instruction!(Set0A, $opcode);
        return_if_is_instruction!(Set1B, $opcode);
        return_if_is_instruction!(Set1C, $opcode);
        return_if_is_instruction!(Set1D, $opcode);
        return_if_is_instruction!(Set1E, $opcode);
        return_if_is_instruction!(Set1H, $opcode);
        return_if_is_instruction!(Set1L, $opcode);
        return_if_is_instruction!(Set1A, $opcode);
        return_if_is_instruction!(Set2B, $opcode);
        return_if_is_instruction!(Set2C, $opcode);
        return_if_is_instruction!(Set2D, $opcode);
        return_if_is_instruction!(Set2E, $opcode);
        return_if_is_instruction!(Set2H, $opcode);
        return_if_is_instruction!(Set2L, $opcode);
        return_if_is_instruction!(Set2A, $opcode);
        return_if_is_instruction!(Set3B, $opcode);
        return_if_is_instruction!(Set3C, $opcode);
        return_if_is_instruction!(Set3D, $opcode);
        return_if_is_instruction!(Set3E, $opcode);
        return_if_is_instruction!(Set3H, $opcode);
        return_if_is_instruction!(Set3L, $opcode);
        return_if_is_instruction!(Set3A, $opcode);
        return_if_is_instruction!(Set4B, $opcode);
        return_if_is_instruction!(Set4C, $opcode);
        return_if_is_instruction!(Set4D, $opcode);
        return_if_is_instruction!(Set4E, $opcode);
        return_if_is_instruction!(Set4H, $opcode);
        return_if_is_instruction!(Set4L, $opcode);
        return_if_is_instruction!(Set4A, $opcode);
        return_if_is_instruction!(Set5B, $opcode);
        return_if_is_instruction!(Set5C, $opcode);
        return_if_is_instruction!(Set5D, $opcode);
        return_if_is_instruction!(Set5E, $opcode);
        return_if_is_instruction!(Set5H, $opcode);
        return_if_is_instruction!(Set5L, $opcode);
        return_if_is_instruction!(Set5A, $opcode);
        return_if_is_instruction!(Set6B, $opcode);
        return_if_is_instruction!(Set6C, $opcode);
        return_if_is_instruction!(Set6D, $opcode);
        return_if_is_instruction!(Set6E, $opcode);
        return_if_is_instruction!(Set6H, $opcode);
        return_if_is_instruction!(Set6L, $opcode);
        return_if_is_instruction!(Set6A, $opcode);
        return_if_is_instruction!(Set7B, $opcode);
        return_if_is_instruction!(Set7C, $opcode);
        return_if_is_instruction!(Set7D, $opcode);
        return_if_is_instruction!(Set7E, $opcode);
        return_if_is_instruction!(Set7H, $opcode);
        return_if_is_instruction!(Set7L, $opcode);
        return_if_is_instruction!(Set7A, $opcode);
    }
}
