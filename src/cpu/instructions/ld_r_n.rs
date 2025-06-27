use paste::paste;
use crate::memory::{MemoryController, MemoryTrait};
use std::sync::Arc;
use parking_lot::Mutex;
use crate::cpu::alu::ALU;
use crate::cpu::instructions::Instruction;
use crate::cpu::register::Register;
use crate::cpu::registers::Registers;


macro_rules! ld_r_n {
    ($opcode:literal, $register:ident, $register_upper:ident) => {
        paste!{
            pub struct [<Ld $register_upper N>] {
                counter: u8
            }

            impl Instruction for [<Ld $register_upper N>] {
                #[inline]
                fn from_opcode(opcode: &u8) -> Option<Box<dyn Instruction>> {
                    if *opcode == $opcode {
                        return Some(Box::new([<Ld $register_upper N>]{ counter: 1 }))
                    }
                    None
                }

                fn get_opcode(&self) -> u8 {
                    $opcode
                }

                fn act(&mut self, registers: &mut Registers, _alu: &mut ALU, memory_controller: Arc<Mutex<MemoryController>>, _enable_interrupts: &mut bool) -> bool {
                    if self.counter == 1 {
                        registers.$register.borrow_mut().set_value(memory_controller.lock().get(registers.pc.get_value()));
                        registers.pc.increment();
                    }
                    else if self.counter == 0 {
                        return true;
                    }

                    self.counter -= 1;
                    false
                }
            }


            #[cfg(test)]
            mod [<ld_ $register n_tests>] {
                use super::*;

                reusable_testing_macro!($opcode, [<Ld $register_upper N>]);

                #[test]
                fn load_n_into_a_on_tick_1() {
                    let mut registers = Registers::new(0, 0, 0, 0, 0xC000, 0);
                    let memory = Arc::new(Mutex::new(MemoryController::new()));
                    let mut alu = ALU::new(registers.f.clone());

                    memory.lock().set(0xC000, 0x12);

                    let mut instruction = [<Ld $register_upper N>] { counter: 1 };

                    let result = instruction.act(&mut registers, &mut alu, memory.clone(), &mut false);

                    assert_eq!(false, result);
                    assert_eq!(0x12, registers.$register.borrow_mut().get_value());
                    assert_eq!(0xC001, registers.pc.get_value());
                }

                #[test]
                fn on_tick_0_get_next_instruction() {
                    let mut registers = Registers::new(0, 0, 0, 0, 0xC001, 0);
                    let memory = Arc::new(Mutex::new(MemoryController::new()));
                    let mut alu = ALU::new(registers.f.clone());

                    let mut instruction = [<Ld $register_upper N>] { counter: 0 };

                    let result = instruction.act(&mut registers, &mut alu, memory.clone(), &mut false);

                    assert_eq!(true, result);
                    assert_eq!(0xC001, registers.pc.get_value());
                }
            }
        }
    }
}

ld_r_n!(0x3E, a, A);
ld_r_n!(0x06, b, B);
ld_r_n!(0x0E, c, C);
ld_r_n!(0x16, d, D);
ld_r_n!(0x1E, e, E);
ld_r_n!(0x2E, h, H);
ld_r_n!(0x2E, l, L);

#[macro_export] macro_rules! ld_r_n_decode_instruction {
    ($opcode:expr) => {
        return_if_is_instruction!(LdAN, $opcode);   //0x3E
        return_if_is_instruction!(LdBN, $opcode);   //0x06
        return_if_is_instruction!(LdCN, $opcode);   //0x0E
        return_if_is_instruction!(LdDN, $opcode);   //0x16
        return_if_is_instruction!(LdEN, $opcode);   //0x1E
        return_if_is_instruction!(LdHN, $opcode);   //0x2E
        return_if_is_instruction!(LdLN, $opcode);   //0x2E
    }
}
