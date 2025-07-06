use crate::cpu::alu::ALU;
use crate::cpu::instructions::Instruction;
use crate::cpu::register::Register;
use crate::cpu::registers::Registers;
use crate::memory::{MemoryController, MemoryTrait};
use parking_lot::Mutex;
use paste::paste;
use std::sync::Arc;


macro_rules! adc_a_r {
    ($opcode:literal, $register:ident, $register_upper:ident) => {
        paste! {
            pub struct [<AdcA $register_upper>] {}


            impl Instruction for [<AdcA $register_upper>] {

                #[inline]
                fn from_opcode(opcode: &u8) -> Option<Box<dyn Instruction>> {
                    if *opcode == $opcode {
                        return Some(Box::new([<AdcA $register_upper>] {}))
                    }
                    None
                }


                fn get_opcode(&self) -> u8 {
                    $opcode
                }

                fn act(&mut self, registers: &mut Registers, alu: &mut ALU, _memory_controller: Arc<Mutex<MemoryController>>, _enable_interrupts: &mut bool, _is_halted: &mut bool) -> bool {
                    alu.adc(&mut *registers.a.clone().borrow_mut(), &registers.$register.clone().borrow());

                    true
                }
            }


            #[cfg(test)]
            mod [<adc_a_ $register>] {
                use super::*;

                reusable_testing_macro!($opcode, [<AdcA $register_upper>]);

                #[test]
                fn adc_r_to_a_on_tick() {
                    let mut registers = Registers::new(0, 0, 0, 0, 0xC000, 0);
                    registers.a.borrow_mut().set_value(0x12);
                    registers.f.borrow_mut().set_bit(ALU::CARRY_FLAG, true);
                    registers.$register.borrow_mut().set_value(0x12);

                    let memory = Arc::new(Mutex::new(MemoryController::new()));
                    let mut alu = ALU::new(registers.f.clone());

                    let mut instruction = [<AdcA $register_upper>] {};

                    let result = instruction.act(&mut registers, &mut alu, memory.clone(), &mut false, &mut false);

                    assert_eq!(true, result);
                    assert_eq!(0x25, registers.a.borrow().get_value());
                }
            }
        }
    }
}


adc_a_r!(0x88, b, B);
adc_a_r!(0x89, c, C);
adc_a_r!(0x8A, d, D);
adc_a_r!(0x8B, e, E);
adc_a_r!(0x8C, h, H);
adc_a_r!(0x8D, l, L);


#[macro_export] macro_rules! adc_a_r_decode_instruction {
    ($opcode:expr) => {
        return_if_is_instruction!(AdcAB, $opcode);   //0x80
        return_if_is_instruction!(AdcAC, $opcode);   //0x81
        return_if_is_instruction!(AdcAD, $opcode);   //0x82
        return_if_is_instruction!(AdcAE, $opcode);   //0x83
        return_if_is_instruction!(AdcAH, $opcode);   //0x84
        return_if_is_instruction!(AdcAL, $opcode);   //0x85
    }
}
