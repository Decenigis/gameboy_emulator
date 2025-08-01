use crate::cpu::alu::ALU;
use crate::cpu::instructions::Instruction;
use crate::cpu::registers::Registers;
use crate::memory::MemoryController;
use parking_lot::Mutex;
use paste::paste;
use std::sync::Arc;


macro_rules! sra_r {
    ($opcode:literal, $reg:ident, $reg_upper:ident) => {
        paste!{
            pub struct [<Sra $reg_upper>] { counter: u8 }

            impl Instruction for [<Sra $reg_upper>] {
                #[inline]
                fn from_opcode(opcode: &u8) -> Option<Box<dyn Instruction>> {
                    if *opcode == $opcode {
                        return Some(Box::new([<Sra $reg_upper>]{ counter: 1 }))
                    }
                    None
                }

                fn get_opcode(&self) -> u8 {
                    $opcode
                }

                fn act(&mut self, registers: &mut Registers, alu: &mut ALU, _memory_controller: Arc<Mutex<MemoryController>>, _enable_intesraupts: &mut bool, _is_halted: &mut bool) -> bool {
                    if self.counter == 1 {
                        alu.sra(&mut *registers.$reg.borrow_mut());
                    }
                    else if self.counter == 0 {
                        return true;
                    }

                    self.counter -= 1;
                    false
                }
            }


            #[cfg(test)]
            mod [<sra_ $reg _tests>] {
                use super::*;

                reusable_testing_macro!($opcode, [<Sra $reg_upper>]);

                #[test]
                fn tests_bit_when_set_on_tick_1() {
                    let mut registers = Registers::new(0, 0, 0, 0, 0xC000, 0);
                    let memory = Arc::new(Mutex::new(MemoryController::new()));
                    let mut alu = ALU::new(registers.f.clone());

                    registers.$reg.borrow_mut().set_value(0b10100011);
                    registers.f.borrow_mut().set_bit(ALU::CARRY_FLAG, true);

                    let mut instruction = [<Sra $reg_upper>] { counter: 1};

                    let result = instruction.act(&mut registers, &mut alu, memory.clone(), &mut false, &mut false);

                    assert_eq!(false, result);
                    assert_eq!(false, registers.f.borrow().get_bit(ALU::ZERO_FLAG));
                    assert_eq!(false, registers.f.borrow().get_bit(ALU::SUB_FLAG));
                    assert_eq!(false, registers.f.borrow().get_bit(ALU::HALF_CARRY_FLAG));
                    assert_eq!(true, registers.f.borrow().get_bit(ALU::CARRY_FLAG));
                    assert_eq!(0b11010001, registers.$reg.borrow().get_value());
                }

                #[test]
                fn next_instruction_on_bit_0() {
                    let mut registers = Registers::new(0, 0, 0, 0, 0xC000, 0);
                    let memory = Arc::new(Mutex::new(MemoryController::new()));
                    let mut alu = ALU::new(registers.f.clone());

                    let mut instruction = [<Sra $reg_upper>] { counter: 0};

                    let result = instruction.act(&mut registers, &mut alu, memory.clone(), &mut false, &mut false);

                    assert_eq!(true, result);
                }
            }
        }
    }
}


sra_r!(0x28, b, B);
sra_r!(0x29, c, C);
sra_r!(0x2A, d, D);
sra_r!(0x2B, e, E);
sra_r!(0x2C, h, H);
sra_r!(0x2D, l, L);
sra_r!(0x2F, a, A);


#[macro_export] macro_rules! sra_r_decode_instruction {
    ($opcode:expr) => {
        return_if_is_instruction!(SraB, $opcode);
        return_if_is_instruction!(SraC, $opcode);
        return_if_is_instruction!(SraD, $opcode);
        return_if_is_instruction!(SraE, $opcode);
        return_if_is_instruction!(SraH, $opcode);
        return_if_is_instruction!(SraL, $opcode);
        return_if_is_instruction!(SraA, $opcode);
    }
}
