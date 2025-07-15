use paste::paste;
use crate::cpu::alu::ALU;
use crate::cpu::instructions::Instruction;
use crate::cpu::register::Register;
use crate::cpu::registers::Registers;
use crate::memory::MemoryController;
use parking_lot::Mutex;
use std::sync::Arc;



macro_rules! dec_rr {
    ($opcode:literal, $register:ident, $register_upper:ident) => {
        paste! {
            pub struct [<Dec $register_upper>] {
                counter: u8,
            }

            impl Instruction for [<Dec $register_upper>] {

                #[inline]
                fn from_opcode(opcode: &u8) -> Option<Box<dyn Instruction>> {
                    if *opcode == $opcode {
                        return Some(Box::new([<Dec $register_upper>] { counter: 1 }))
                    }
                    None
                }

                fn get_opcode(&self) -> u8 {
                    $opcode
                }

                fn act(&mut self, registers: &mut Registers, _alu: &mut ALU, _memory_controller: Arc<Mutex<MemoryController>>, _enable_interrupts: &mut bool, _is_halted: &mut bool) -> bool {
                    if self.counter == 1 {
                        registers.$register.decrement();
                    }
                    else if self.counter == 0 {
                        return true;
                    }

                    self.counter -= 1;
                    false
                }
            }



            #[cfg(test)]
            mod [<dec_ $register _tests>] {
                use super::*;
                use crate::cpu::register::Register;

                reusable_testing_macro!($opcode, [<Dec $register_upper>]);

                #[test]
                fn decrement_register_on_tick_1() {
                    let mut registers = Registers::new(0, 0, 0, 0, 0xC000, 0);
                    let mut alu = ALU::new(registers.f.clone());
                    let memory = Arc::new(Mutex::new(MemoryController::new()));

                    registers.$register.set_value(0x1234);

                    let mut instruction = [<Dec $register_upper>] { counter: 1 };

                    let result = instruction.act(&mut registers, &mut alu, memory.clone(),&mut false, &mut false);

                    assert_eq!(false, result);
                    assert_eq!(0x1233, registers.$register.get_value());
                }

                #[test]
                fn get_next_instruction_on_tick_0() {
                    let mut registers = Registers::new(0, 0, 0, 0, 0xC000, 0);
                    let mut alu = ALU::new(registers.f.clone());
                    let memory = Arc::new(Mutex::new(MemoryController::new()));

                    let mut instruction = [<Dec $register_upper>] { counter: 0 };

                    let result = instruction.act(&mut registers, &mut alu, memory.clone(),&mut false, &mut false);

                    assert_eq!(true, result);
                }
            }
        }
    }
}

dec_rr!(0x0B, bc, Bc);
dec_rr!(0x1B, de, De);
dec_rr!(0x2B, hl, Hl);
dec_rr!(0x3B, sp, Sp);


#[macro_export] macro_rules! dec_rr_decode_instruction {
    ($opcode:expr) => {
        return_if_is_instruction!(DecBc, $opcode);   //0x0B
        return_if_is_instruction!(DecDe, $opcode);   //0x1B
        return_if_is_instruction!(DecHl, $opcode);   //0x2B
        return_if_is_instruction!(DecSp, $opcode);   //0x3B
    }
}
