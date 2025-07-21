use crate::cpu::alu::ALU;
use crate::cpu::instructions::Instruction;
use crate::cpu::registers::Registers;
use crate::memory::MemoryController;
use parking_lot::Mutex;
use paste::paste;
use std::sync::Arc;


macro_rules! and_a_r {
    ($opcode:literal, $register:ident, $register_upper:ident) => {
        paste! {
            pub struct [<AndA $register_upper>] {}


            impl Instruction for [<AndA $register_upper>] {

                #[inline]
                fn from_opcode(opcode: &u8) -> Option<Box<dyn Instruction>> {
                    if *opcode == $opcode {
                        return Some(Box::new([<AndA $register_upper>] {}))
                    }
                    None
                }


                fn get_opcode(&self) -> u8 {
                    $opcode
                }

                fn act(&mut self, registers: &mut Registers, alu: &mut ALU, _memory_controller: Arc<Mutex<MemoryController>>, _enable_interrupts: &mut bool, _is_halted: &mut bool) -> bool {
                    alu.and(&mut *registers.a.clone().borrow_mut(), &registers.$register.clone().borrow());

                    true
                }
            }


            #[cfg(test)]
            mod [<and_a_ $register>] {
                use super::*;
                use crate::cpu::register::Register;
                

                reusable_testing_macro!($opcode, [<AndA $register_upper>]);

                #[test]
                fn and_r_to_a_on_tick() {
                    let mut registers = Registers::new(0, 0, 0, 0, 0xC000, 0);
                    registers.a.borrow_mut().set_value(0x36);
                    registers.$register.borrow_mut().set_value(0x12);

                    let memory = Arc::new(Mutex::new(MemoryController::new()));
                    let mut alu = ALU::new(registers.f.clone());

                    let mut instruction = [<AndA $register_upper>] {};

                    let result = instruction.act(&mut registers, &mut alu, memory.clone(), &mut false, &mut false);

                    assert_eq!(true, result);
                    assert_eq!(0x12, registers.a.borrow().get_value());
                }
            }
        }
    }
}


and_a_r!(0xA0, b, B);
and_a_r!(0xA1, c, C);
and_a_r!(0xA2, d, D);
and_a_r!(0xA3, e, E);
and_a_r!(0xA4, h, H);
and_a_r!(0xA5, l, L);


#[macro_export] macro_rules! and_a_r_decode_instruction {
    ($opcode:expr) => {
        return_if_is_instruction!(AndAB, $opcode);   //0xA0
        return_if_is_instruction!(AndAC, $opcode);   //0xA1
        return_if_is_instruction!(AndAD, $opcode);   //0xA2
        return_if_is_instruction!(AndAE, $opcode);   //0xA3
        return_if_is_instruction!(AndAH, $opcode);   //0xA4
        return_if_is_instruction!(AndAL, $opcode);   //0xA5
    }
}
