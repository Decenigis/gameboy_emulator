use crate::cpu::alu::ALU;
use crate::cpu::instructions::Instruction;
use crate::cpu::registers::Registers;
use crate::memory::MemoryController;
use parking_lot::Mutex;
use paste::paste;
use std::sync::Arc;


macro_rules! sub_a_r {
    ($opcode:literal, $register:ident, $register_upper:ident) => {
        paste! {
            pub struct [<SubA $register_upper>] {}


            impl Instruction for [<SubA $register_upper>] {

                #[inline]
                fn from_opcode(opcode: &u8) -> Option<Box<dyn Instruction>> {
                    if *opcode == $opcode {
                        return Some(Box::new([<SubA $register_upper>] {}))
                    }
                    None
                }


                fn get_opcode(&self) -> u8 {
                    $opcode
                }

                fn act(&mut self, registers: &mut Registers, alu: &mut ALU, _memory_controller: Arc<Mutex<MemoryController>>, _enable_interrupts: &mut bool, _is_halted: &mut bool) -> bool {
                    alu.sub(&mut *registers.a.clone().borrow_mut(), &registers.$register.clone().borrow());

                    true
                }
            }


            #[cfg(test)]
            mod [<sub_a_ $register>] {
                use super::*;
                use crate::cpu::register::Register;

                reusable_testing_macro!($opcode, [<SubA $register_upper>]);

                #[test]
                fn sub_r_to_a_on_tick() {
                    let mut registers = Registers::new(0, 0, 0, 0, 0xC000, 0);
                    registers.a.borrow_mut().set_value(0x24);
                    registers.$register.borrow_mut().set_value(0x12);

                    let memory = Arc::new(Mutex::new(MemoryController::new()));
                    let mut alu = ALU::new(registers.f.clone());

                    let mut instruction = [<SubA $register_upper>] {};

                    let result = instruction.act(&mut registers, &mut alu, memory.clone(), &mut false, &mut false);

                    assert_eq!(true, result);
                    assert_eq!(0x12, registers.a.borrow().get_value());
                }
            }
        }
    }
}


sub_a_r!(0x90, b, B);
sub_a_r!(0x91, c, C);
sub_a_r!(0x92, d, D);
sub_a_r!(0x93, e, E);
sub_a_r!(0x94, h, H);
sub_a_r!(0x95, l, L);


#[macro_export] macro_rules! sub_a_r_decode_instruction {
    ($opcode:expr) => {
        return_if_is_instruction!(SubAB, $opcode);   //0x90
        return_if_is_instruction!(SubAC, $opcode);   //0x91
        return_if_is_instruction!(SubAD, $opcode);   //0x92
        return_if_is_instruction!(SubAE, $opcode);   //0x93
        return_if_is_instruction!(SubAH, $opcode);   //0x94
        return_if_is_instruction!(SubAL, $opcode);   //0x95
    }
}
