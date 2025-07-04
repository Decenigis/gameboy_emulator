use crate::cpu::instructions::Instruction;
use crate::memory::MemoryController;
use crate::cpu::register8::Register8;
use crate::cpu::registers::Registers;
use crate::cpu::register::Register;
use crate::cpu::alu::ALU;
use std::sync::Arc;
use parking_lot::Mutex;
use paste::paste;

macro_rules! dec_r {
    ($opcode:literal, $register:ident, $register_upper:ident) => {
        paste!{
            pub struct [<Dec $register_upper>] {}

            impl Instruction for [<Dec $register_upper>] {
                #[inline]
                fn from_opcode(opcode: &u8) -> Option<Box<dyn Instruction>> {
                    if *opcode == $opcode {
                        return Some(Box::new([<Dec $register_upper>] {}))
                    }
                    None
                }

                fn get_opcode(&self) -> u8 {
                    $opcode
                }

                fn act(&mut self, registers: &mut Registers, alu: &mut ALU, _memory_controller: Arc<Mutex<MemoryController>>, _enable_interrupts: &mut bool) -> bool {
                    alu.sub_no_carry(&mut *registers.$register.clone().borrow_mut(), &Register8::one());

                    true
                }
            }

            #[cfg(test)]
            mod [<dec_ $register _tests>] {
                use super::*;

                reusable_testing_macro!($opcode, [<Dec $register_upper>]);

                #[test]
                fn decrement_r_on_tick() {
                    let mut registers = Registers::new(0, 0, 0, 0, 0xC000, 0);
                    registers.$register.borrow_mut().set_value(0x12);

                    let memory = Arc::new(Mutex::new(MemoryController::new()));
                    let mut alu = ALU::new(registers.f.clone());

                    let mut instruction = [<Dec $register_upper>] {};

                    let result = instruction.act(&mut registers, &mut alu, memory.clone(), &mut false);

                    assert_eq!(true, result);
                    assert_eq!(0x11, registers.$register.borrow().get_value());
                }
            }
        }
    }
}

dec_r!(0x3D, a, A);
dec_r!(0x05, b, B);
dec_r!(0x0D, c, C);
dec_r!(0x15, d, D);
dec_r!(0x1D, e, E);
dec_r!(0x25, h, H);
dec_r!(0x2D, l, L);


#[macro_export] macro_rules! dec_r_decode_instruction {
    //this really sucks
    ($opcode:expr) => {
        return_if_is_instruction!(DecA, $opcode);
        return_if_is_instruction!(DecB, $opcode);
        return_if_is_instruction!(DecC, $opcode);
        return_if_is_instruction!(DecD, $opcode);
        return_if_is_instruction!(DecE, $opcode);
        return_if_is_instruction!(DecH, $opcode);
        return_if_is_instruction!(DecL, $opcode);
    }
}
