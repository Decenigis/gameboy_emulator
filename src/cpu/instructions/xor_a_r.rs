use crate::cpu::alu::ALU;
use crate::cpu::instructions::Instruction;
use crate::cpu::registers::Registers;
use crate::memory::MemoryController;
use parking_lot::Mutex;
use paste::paste;
use std::sync::Arc;


macro_rules! xor_a_r {
    ($opcode:literal, $register:ident, $register_upper:ident) => {
        paste! {
            pub struct [<XorA $register_upper>] {}


            impl Instruction for [<XorA $register_upper>] {

                #[inline]
                fn from_opcode(opcode: &u8) -> Option<Box<dyn Instruction>> {
                    if *opcode == $opcode {
                        return Some(Box::new([<XorA $register_upper>] {}))
                    }
                    None
                }


                fn get_opcode(&self) -> u8 {
                    $opcode
                }

                fn act(&mut self, registers: &mut Registers, alu: &mut ALU, _memory_controller: Arc<Mutex<MemoryController>>, _enable_interrupts: &mut bool, _is_halted: &mut bool) -> bool {
                    alu.xor_internal(registers.a.clone(), registers.$register.clone());

                    true
                }
            }


            #[cfg(test)]
            mod [<xor_a_ $register>] {
                use super::*;
                use crate::cpu::register::Register;
                use crate::memory::MemoryTrait;

                reusable_testing_macro!($opcode, [<XorA $register_upper>]);

                #[test]
                fn xor_r_to_a_on_tick() {
                    let mut registers = Registers::new(0, 0, 0, 0, 0xC000, 0);
                    registers.a.borrow_mut().set_value(0x36);
                    registers.$register.borrow_mut().set_value(0x12);

                    let memory = Arc::new(Mutex::new(MemoryController::new()));
                    let mut alu = ALU::new(registers.f.clone());

                    let mut instruction = [<XorA $register_upper>] {};

                    let result = instruction.act(&mut registers, &mut alu, memory.clone(), &mut false, &mut false);

                    assert_eq!(true, result);
                    assert_eq!(0x24, registers.a.borrow().get_value());
                }
            }
        }
    }
}


xor_a_r!(0xA8, b, B);
xor_a_r!(0xA9, c, C);
xor_a_r!(0xAA, d, D);
xor_a_r!(0xAB, e, E);
xor_a_r!(0xAC, h, H);
xor_a_r!(0xAD, l, L);


#[macro_export] macro_rules! xor_a_r_decode_instruction {
    ($opcode:expr) => {
        return_if_is_instruction!(XorAB, $opcode);   //0xA8
        return_if_is_instruction!(XorAC, $opcode);   //0xA9
        return_if_is_instruction!(XorAD, $opcode);   //0xAA
        return_if_is_instruction!(XorAE, $opcode);   //0xAB
        return_if_is_instruction!(XorAH, $opcode);   //0xAC
        return_if_is_instruction!(XorAL, $opcode);   //0xAD
    }
}
