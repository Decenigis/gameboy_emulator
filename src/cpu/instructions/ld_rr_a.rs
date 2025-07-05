use paste::paste;
use std::sync::Arc;
use parking_lot::Mutex;
use crate::cpu::alu::ALU;
use crate::cpu::instructions::Instruction;
use crate::cpu::register::Register;
use crate::cpu::registers::Registers;
use crate::memory::{MemoryController, MemoryTrait};

macro_rules! ld_rr_a {
    ($opcode:literal, $register:ident, $register_upper:ident) => {
        paste!{
            pub struct [<Ld $register_upper A>] {
                counter: u8
            }

            impl Instruction for [<Ld $register_upper A>]  {

                #[inline]
                fn from_opcode(opcode: &u8) -> Option<Box<dyn Instruction>> {
                    if *opcode == $opcode {
                        return Some(Box::new([<Ld $register_upper A>]  { counter: 1 }))
                    }
                    None
                }

                fn get_opcode(&self) -> u8 {
                    $opcode
                }

                fn act(&mut self, registers: &mut Registers, _alu: &mut ALU, memory_controller: Arc<Mutex<MemoryController>>, _enable_interrupts: &mut bool) -> bool {
                    if self.counter == 1 {
                        memory_controller.lock().set(registers.$register.get_value(), registers.a.borrow().get_value());
                    }
                    else if self.counter == 0 {
                        return true;
                    }

                    self.counter -= 1;
                    false
                }
            }

            #[cfg(test)]
            mod [<ld_ $register _A_tests>] {
                use super::*;

                reusable_testing_macro!($opcode, [<Ld $register_upper A>] );

                #[test]
                fn set_value_on_tick_1() {
                    let mut registers = Registers::new(0x1200, 0, 0, 0, 0xC000, 0);
                    let mut alu = ALU::new(registers.f.clone());
                    let memory = Arc::new(Mutex::new(MemoryController::new()));

                    registers.$register.set_value(0xC000);

                    let mut instruction = [<Ld $register_upper A>]  { counter: 1 };

                    let result = instruction.act(&mut registers, &mut alu, memory.clone(),&mut false);

                    assert_eq!(false, result);
                    assert_eq!(0x12, memory.lock().get(0xC000));
                }

                #[test]
                fn get_next_instruction_on_tick_0() {
                    let mut registers = Registers::new(0, 0, 0, 0, 0xC000, 0);
                    let mut alu = ALU::new(registers.f.clone());
                    let memory = Arc::new(Mutex::new(MemoryController::new()));

                    let mut instruction = [<Ld $register_upper A>]  { counter: 0 };

                    let result = instruction.act(&mut registers, &mut alu, memory.clone(),&mut false);

                    assert_eq!(true, result);
                }
            }
        }
    }
}

ld_rr_a!(0x02, bc, Bc);
ld_rr_a!(0x12, de, De);
ld_rr_a!(0x77, hl, Hl);

#[macro_export] macro_rules! ld_rr_a_decode_instruction {
    ($opcode:expr) => {
        return_if_is_instruction!(LdBcA, $opcode);
        return_if_is_instruction!(LdDeA, $opcode);
        return_if_is_instruction!(LdHlA, $opcode);
    }
}
