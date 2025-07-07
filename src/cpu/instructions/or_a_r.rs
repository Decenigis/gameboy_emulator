use paste::paste;
use crate::cpu::alu::ALU;
use crate::cpu::instructions::Instruction;
use crate::cpu::registers::Registers;
use crate::memory::MemoryController;
use parking_lot::Mutex;
use std::sync::Arc;



macro_rules! or_a_r {
    ($opcode:literal, $register:ident, $register_upper:ident) => {
        paste! {
            pub struct [<OrA $register_upper>] {}

            impl Instruction for [<OrA $register_upper>] {

                #[inline]
                fn from_opcode(opcode: &u8) -> Option<Box<dyn Instruction>> {
                    if *opcode == $opcode {
                        return Some(Box::new([<OrA $register_upper>] {}))
                    }
                    None
                }

                fn get_opcode(&self) -> u8 {
                    $opcode
                }

                fn act(&mut self, registers: &mut Registers, alu: &mut ALU, _memory_controller: Arc<Mutex<MemoryController>>, _enable_interrupts: &mut bool, _is_halted: &mut bool) -> bool {
                    alu.or_internal(registers.a.clone(), registers.$register.clone());

                    true
                }
            }



            #[cfg(test)]
            mod [<or_a_ $register _tests>] {
                use super::*;
                use crate::cpu::register::Register;

                reusable_testing_macro!($opcode, [<OrA $register_upper>]);

                #[test]
                fn act_immediately_returns_true() {
                    let mut registers = Registers::new(0, 0, 0, 0, 0, 0);
                    let mut alu = ALU::new(registers.f.clone());

                    let mut instruction = [<OrA $register_upper>] {};

                    let result = instruction.act(&mut registers, &mut alu, Arc::new(Mutex::new(MemoryController::new())) ,&mut false, &mut false);

                    assert_eq!(true, result);
                }

                #[test]
                fn act_ors_correctly() {
                    let mut registers = Registers::new(0, 0, 0, 0, 0, 0);
                    registers.a.borrow_mut().set_value(0xF0);
                    registers.$register.borrow_mut().set_value(0x0F);
                    let mut alu = ALU::new(registers.f.clone());

                    let mut instruction = [<OrA $register_upper>] {};

                    instruction.act(&mut registers, &mut alu, Arc::new(Mutex::new(MemoryController::new())), &mut false, &mut false);

                    assert_eq!(0xFF, registers.a.borrow().get_value());
                }
            }
        }
    }
}

or_a_r!(0xB0, b, B);
or_a_r!(0xB1, c, C);
or_a_r!(0xB2, d, D);
or_a_r!(0xB3, e, E);
or_a_r!(0xB4, h, H);
or_a_r!(0xB5, l, L);


#[macro_export] macro_rules! or_a_r_decode_instruction {
    ($opcode:expr) => {
        return_if_is_instruction!(OrAB, $opcode);   //0xB0
        return_if_is_instruction!(OrAC, $opcode);   //0xB1
        return_if_is_instruction!(OrAD, $opcode);   //0xB2
        return_if_is_instruction!(OrAE, $opcode);   //0xB3
        return_if_is_instruction!(OrAH, $opcode);   //0xB4
        return_if_is_instruction!(OrAL, $opcode);   //0xB5

    }
}
