use crate::cpu::alu::ALU;
use crate::cpu::instructions::Instruction;
use crate::cpu::registers::Registers;
use crate::memory::MemoryController;
use parking_lot::Mutex;
use paste::paste;
use std::sync::Arc;


macro_rules! sbc_a_r {
    ($opcode:literal, $register:ident, $register_upper:ident) => {
        paste! {
            pub struct [<SbcA $register_upper>] {}


            impl Instruction for [<SbcA $register_upper>] {

                #[inline]
                fn from_opcode(opcode: &u8) -> Option<Box<dyn Instruction>> {
                    if *opcode == $opcode {
                        return Some(Box::new([<SbcA $register_upper>] {}))
                    }
                    None
                }


                fn get_opcode(&self) -> u8 {
                    $opcode
                }

                fn act(&mut self, registers: &mut Registers, alu: &mut ALU, _memory_controller: Arc<Mutex<MemoryController>>, _enable_interrupts: &mut bool, _is_halted: &mut bool) -> bool {
                    alu.sbc(&mut *registers.a.clone().borrow_mut(), &registers.$register.clone().borrow());

                    true
                }
            }


            #[cfg(test)]
            mod [<sbc_a_ $register>] {
                use super::*;
                use crate::cpu::register::Register;

                reusable_testing_macro!($opcode, [<SbcA $register_upper>]);

                #[test]
                fn sbc_r_to_a_on_tick() {
                    let mut registers = Registers::new(0, 0, 0, 0, 0xC000, 0);
                    registers.a.borrow_mut().set_value(0x24);
                    registers.$register.borrow_mut().set_value(0x12);
                    registers.f.borrow_mut().set_bit(ALU::CARRY_FLAG, true);

                    let memory = Arc::new(Mutex::new(MemoryController::new()));
                    let mut alu = ALU::new(registers.f.clone());


                    let mut instruction = [<SbcA $register_upper>] {};


                    let result = instruction.act(&mut registers, &mut alu, memory.clone(), &mut false, &mut false);

                    assert_eq!(true, result);
                    assert_eq!(0x11, registers.a.borrow().get_value());
                }
            }
        }
    }
}


sbc_a_r!(0x98, b, B);
sbc_a_r!(0x99, c, C);
sbc_a_r!(0x9A, d, D);
sbc_a_r!(0x9B, e, E);
sbc_a_r!(0x9C, h, H);
sbc_a_r!(0x9D, l, L);


#[macro_export] macro_rules! sbc_a_r_decode_instruction {
    ($opcode:expr) => {
        return_if_is_instruction!(SbcAB, $opcode);   //0x98
        return_if_is_instruction!(SbcAC, $opcode);   //0x99
        return_if_is_instruction!(SbcAD, $opcode);   //0x9A
        return_if_is_instruction!(SbcAE, $opcode);   //0x9B
        return_if_is_instruction!(SbcAH, $opcode);   //0x9C
        return_if_is_instruction!(SbcAL, $opcode);   //0x9D
    }
}
