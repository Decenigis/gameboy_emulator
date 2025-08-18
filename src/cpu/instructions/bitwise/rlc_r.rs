use crate::cpu::alu::ALU;
use crate::cpu::instructions::Instruction;
use crate::cpu::register::Register;
use crate::cpu::registers::Registers;
use crate::memory::MemoryController;
use parking_lot::Mutex;
use paste::paste;
use std::sync::Arc;


macro_rules! rlc_r {
    ($opcode:literal, $reg:ident, $reg_upper:ident) => {
        paste!{
            pub struct [<Rlc $reg_upper>] { counter: u8 }

            impl Instruction for [<Rlc $reg_upper>] {
                #[inline]
                fn from_opcode(opcode: &u8) -> Option<Box<dyn Instruction>> {
                    if *opcode == $opcode {
                        return Some(Box::new([<Rlc $reg_upper>]{ counter: 1 }))
                    }
                    None
                }

                fn get_opcode(&self) -> u8 {
                    $opcode
                }

                fn act(&mut self, registers: &mut Registers, alu: &mut ALU, _memory_controller: Arc<Mutex<MemoryController>>, _enable_interlcupts: &mut bool, _is_halted: &mut bool) -> bool {
                    if self.counter == 1 {
                        alu.rlc(&mut *registers.$reg.borrow_mut());
                    }
                    else if self.counter == 0 {
                        return true;
                    }

                    self.counter -= 1;
                    false
                }
            }


            #[cfg(test)]
            mod [<rlc_ $reg _tests>] {
                use super::*;

                #[test]
                fn from_opcode_returns_given_right_opcode() {
                    let instruction = [<Rlc $reg_upper>]::from_opcode(&$opcode);

                    assert_eq!(true, instruction.is_some());
                }

                #[test]
                fn from_opcode_returns_none_given_wrong_opcode() {
                    let instruction = [<Rlc $reg_upper>]::from_opcode(&0xED);

                    assert_eq!(true, instruction.is_none());
                }

                #[test]
                fn get_opcode_returns_opcode() {
                    let instruction = [<Rlc $reg_upper>]::from_opcode(&$opcode).unwrap();

                    assert_eq!($opcode, instruction.get_opcode());
                }


                #[test]
                fn rolls_bit_on_tick_1() {
                    let mut registers = Registers::new(0, 0, 0, 0, 0xC000, 0);
                    let memory = Arc::new(Mutex::new(MemoryController::new()));
                    let mut alu = ALU::new(registers.f.clone());

                    registers.$reg.borrow_mut().set_value(0b00010001);
                    registers.f.borrow_mut().set_bit(ALU::CARRY_FLAG, true);

                    let mut instruction = [<Rlc $reg_upper>] { counter: 1};

                    let result = instruction.act(&mut registers, &mut alu, memory.clone(), &mut false, &mut false);

                    assert_eq!(false, result);
                    assert_eq!(false, registers.f.borrow().get_bit(ALU::ZERO_FLAG));
                    assert_eq!(false, registers.f.borrow().get_bit(ALU::SUB_FLAG));
                    assert_eq!(false, registers.f.borrow().get_bit(ALU::HALF_CARRY_FLAG));
                    assert_eq!(false, registers.f.borrow().get_bit(ALU::CARRY_FLAG));
                    assert_eq!(0b00100010, registers.$reg.borrow().get_value());
                }

                #[test]
                fn next_instruction_on_bit_0() {
                    let mut registers = Registers::new(0, 0, 0, 0, 0xC000, 0);
                    let memory = Arc::new(Mutex::new(MemoryController::new()));
                    let mut alu = ALU::new(registers.f.clone());

                    let mut instruction = [<Rlc $reg_upper>] { counter: 0};

                    let result = instruction.act(&mut registers, &mut alu, memory.clone(), &mut false, &mut false);

                    assert_eq!(true, result);
                }
            }
        }
    }
}


rlc_r!(0x00, b, B);
rlc_r!(0x01, c, C);
rlc_r!(0x02, d, D);
rlc_r!(0x03, e, E);
rlc_r!(0x04, h, H);
rlc_r!(0x05, l, L);
rlc_r!(0x07, a, A);


#[macro_export] macro_rules! rlc_r_decode_instruction {
    ($opcode:expr) => {
        return_if_is_instruction!(RlcB, $opcode);
        return_if_is_instruction!(RlcC, $opcode);
        return_if_is_instruction!(RlcD, $opcode);
        return_if_is_instruction!(RlcE, $opcode);
        return_if_is_instruction!(RlcH, $opcode);
        return_if_is_instruction!(RlcL, $opcode);
        return_if_is_instruction!(RlcA, $opcode);
    }
}
