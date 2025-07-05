use crate::cpu::alu::ALU;
use crate::cpu::instructions::Instruction;
use crate::cpu::register::Register;
use crate::cpu::registers::Registers;
use crate::memory::{MemoryController, MemoryTrait};
use parking_lot::Mutex;
use paste::paste;
use std::sync::Arc;


macro_rules! add_a_r {
    ($opcode:literal, $register:ident, $register_upper:ident) => {
        paste! {
            pub struct [<AddA $register_upper>] {}


            impl Instruction for [<AddA $register_upper>] {

                #[inline]
                fn from_opcode(opcode: &u8) -> Option<Box<dyn Instruction>> {
                    if *opcode == $opcode {
                        return Some(Box::new([<AddA $register_upper>] {}))
                    }
                    None
                }


                fn get_opcode(&self) -> u8 {
                    $opcode
                }

                fn act(&mut self, registers: &mut Registers, alu: &mut ALU, _memory_controller: Arc<Mutex<MemoryController>>, _enable_interrupts: &mut bool, _is_halted: &mut bool) -> bool {
                    alu.add(&mut *registers.a.clone().borrow_mut(), &registers.$register.clone().borrow());

                    true
                }
            }


            #[cfg(test)]
            mod [<add_a_ $register>] {
                use super::*;

                reusable_testing_macro!($opcode, [<AddA $register_upper>]);

                #[test]
                fn add_r_to_a_on_tick() {
                    let mut registers = Registers::new(0, 0, 0, 0, 0xC000, 0);
                    registers.a.borrow_mut().set_value(0x12);
                    registers.$register.borrow_mut().set_value(0x12);

                    let memory = Arc::new(Mutex::new(MemoryController::new()));
                    let mut alu = ALU::new(registers.f.clone());

                    let mut instruction = [<AddA $register_upper>] {};

                    let result = instruction.act(&mut registers, &mut alu, memory.clone(), &mut false, &mut false);

                    assert_eq!(true, result);
                    assert_eq!(0x24, registers.a.borrow().get_value());
                }
            }
        }
    }
}


add_a_r!(0x80, b, B);
add_a_r!(0x81, c, C);
add_a_r!(0x82, d, D);
add_a_r!(0x83, e, E);
add_a_r!(0x84, h, H);
add_a_r!(0x85, l, L);


#[macro_export] macro_rules! add_a_r_decode_instruction {
    ($opcode:expr) => {
        return_if_is_instruction!(AddAB, $opcode);   //0x80
        return_if_is_instruction!(AddAC, $opcode);   //0x81
        return_if_is_instruction!(AddAD, $opcode);   //0x82
        return_if_is_instruction!(AddAE, $opcode);   //0x83
        return_if_is_instruction!(AddAH, $opcode);   //0x84
        return_if_is_instruction!(AddAL, $opcode);   //0x85
    }
}
