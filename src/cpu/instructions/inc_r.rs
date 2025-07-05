use crate::cpu::alu::ALU;
use crate::cpu::instructions::Instruction;
use crate::cpu::register::Register;
use crate::cpu::register8::Register8;
use crate::cpu::registers::Registers;
use crate::memory::MemoryController;
use parking_lot::Mutex;
use paste::paste;
use std::sync::Arc;

macro_rules! inc_r {
    ($opcode:literal, $register:ident, $register_upper:ident) => {
        paste!{
            pub struct [<Inc $register_upper>] {}

            impl Instruction for [<Inc $register_upper>] {
                #[inline]
                fn from_opcode(opcode: &u8) -> Option<Box<dyn Instruction>> {
                    if *opcode == $opcode {
                        return Some(Box::new([<Inc $register_upper>] {}))
                    }
                    None
                }

                fn get_opcode(&self) -> u8 {
                    $opcode
                }

                fn act(&mut self, registers: &mut Registers, alu: &mut ALU, _memory_controller: Arc<Mutex<MemoryController>>, _enable_interrupts: &mut bool, _is_halted: &mut bool) -> bool {
                    alu.add_no_carry(&mut *registers.$register.clone().borrow_mut(), &Register8::one());

                    true
                }
            }

            #[cfg(test)]
            mod [<inc_ $register _tests>] {
                use super::*;

                reusable_testing_macro!($opcode, [<Inc $register_upper>]);

                #[test]
                fn increment_r_on_tick() {
                    let mut registers = Registers::new(0, 0, 0, 0, 0xC000, 0);
                    registers.$register.borrow_mut().set_value(0x12);

                    let memory = Arc::new(Mutex::new(MemoryController::new()));
                    let mut alu = ALU::new(registers.f.clone());

                    let mut instruction = [<Inc $register_upper>] {};

                    let result = instruction.act(&mut registers, &mut alu, memory.clone(), &mut false, &mut false);

                    assert_eq!(true, result);
                    assert_eq!(0x13, registers.$register.borrow().get_value());
                }
            }
        }
    }
}

inc_r!(0x3C, a, A);
inc_r!(0x04, b, B);
inc_r!(0x0C, c, C);
inc_r!(0x14, d, D);
inc_r!(0x1C, e, E);
inc_r!(0x24, h, H);
inc_r!(0x2C, l, L);


#[macro_export] macro_rules! inc_r_decode_instruction {
    //this really sucks
    ($opcode:expr) => {
        return_if_is_instruction!(IncA, $opcode);
        return_if_is_instruction!(IncB, $opcode);
        return_if_is_instruction!(IncC, $opcode);
        return_if_is_instruction!(IncD, $opcode);
        return_if_is_instruction!(IncE, $opcode);
        return_if_is_instruction!(IncH, $opcode);
        return_if_is_instruction!(IncL, $opcode);
    }
}
