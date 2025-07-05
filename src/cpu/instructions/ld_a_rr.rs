use crate::cpu::alu::ALU;
use crate::cpu::instructions::Instruction;
use crate::cpu::register::Register;
use crate::cpu::registers::Registers;
use crate::memory::{MemoryController, MemoryTrait};
use parking_lot::Mutex;
use paste::paste;
use std::sync::Arc;

macro_rules! ld_a_rr {
    ($opcode:literal, $register:ident, $register_upper:ident) => {
        paste!{
            pub struct [<LdA $register_upper>] {
                counter: u8
            }

            impl Instruction for [<LdA $register_upper>]  {

                #[inline]
                fn from_opcode(opcode: &u8) -> Option<Box<dyn Instruction>> {
                    if *opcode == $opcode {
                        return Some(Box::new([<LdA $register_upper>]  { counter: 1 }))
                    }
                    None
                }

                fn get_opcode(&self) -> u8 {
                    $opcode
                }

                fn act(&mut self, registers: &mut Registers, _alu: &mut ALU, memory_controller: Arc<Mutex<MemoryController>>, _enable_interrupts: &mut bool, _is_halted: &mut bool) -> bool {
                    if self.counter == 1 {
                        registers.a.borrow_mut().set_value(memory_controller.lock().get(registers.$register.get_value()));
                    }
                    else if self.counter == 0 {
                        return true;
                    }

                    self.counter -= 1;
                    false
                }
            }

            #[cfg(test)]
            mod [<ld_a_ $register _tests>] {
                use super::*;

                reusable_testing_macro!($opcode, [<LdA $register_upper>] );

                #[test]
                fn set_value_on_tick_1() {
                    let mut registers = Registers::new(0x1200, 0, 0, 0, 0xC000, 0);
                    let mut alu = ALU::new(registers.f.clone());
                    let memory = Arc::new(Mutex::new(MemoryController::new()));

                    registers.$register.set_value(0xC000);
                    memory.lock().set(0xC000, 0x12);

                    let mut instruction = [<LdA $register_upper>]  { counter: 1 };

                    let result = instruction.act(&mut registers, &mut alu, memory.clone(), &mut false, &mut false);

                    assert_eq!(false, result);
                    assert_eq!(0x12, registers.a.borrow().get_value());
                }

                #[test]
                fn get_next_instruction_on_tick_0() {
                    let mut registers = Registers::new(0, 0, 0, 0, 0xC000, 0);
                    let mut alu = ALU::new(registers.f.clone());
                    let memory = Arc::new(Mutex::new(MemoryController::new()));

                    let mut instruction = [<LdA $register_upper>]  { counter: 0 };

                    let result = instruction.act(&mut registers, &mut alu, memory.clone(), &mut false, &mut false);

                    assert_eq!(true, result);
                }
            }
        }
    }
}

ld_a_rr!(0x0A, bc, Bc);
ld_a_rr!(0x1A, de, De);
ld_a_rr!(0x7E, hl, Hl);

#[macro_export] macro_rules! ld_a_rr_decode_instruction {
    ($opcode:expr) => {
        return_if_is_instruction!(LdABc, $opcode);
        return_if_is_instruction!(LdADe, $opcode);
        return_if_is_instruction!(LdAHl, $opcode);
    }
}
