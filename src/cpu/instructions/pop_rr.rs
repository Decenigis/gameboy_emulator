use crate::cpu::alu::ALU;
use crate::cpu::instructions::Instruction;
use crate::cpu::register::Register;
use crate::cpu::registers::Registers;
use crate::memory::{MemoryController, MemoryTrait};
use parking_lot::Mutex;
use paste::paste;
use std::sync::Arc;


macro_rules! pop_rr {
    ($opcode:literal, $register:ident, $register_upper:ident) => {
        paste! {
            pub struct [<Pop $register_upper>] {
                counter: u8
            }

            impl Instruction for [<Pop $register_upper>] {

                #[inline]
                fn from_opcode(opcode: &u8) -> Option<Box<dyn Instruction>> {
                    if *opcode == $opcode {
                        return Some(Box::new([<Pop $register_upper>] { counter: 2 }))
                    }
                    None
                }

                fn get_opcode(&self) -> u8 {
                    $opcode
                }

                //maybe wrong ordering, shouldn't have too much bearing.
                fn act(&mut self, registers: &mut Registers, _alu: &mut ALU, memory_controller: Arc<Mutex<MemoryController>>, _enable_interrupts: &mut bool, _is_halted: &mut bool) -> bool {
                    if self.counter == 2 {
                        registers.$register.set_value(memory_controller.lock().get(registers.sp.get_value()) as u16);
                        registers.sp.increment();

                    }
                    else if self.counter == 1 {
                        let curr_val = registers.$register.get_value() & 0xFF;
                        registers.$register.set_value(curr_val | ((memory_controller.lock().get(registers.sp.get_value()) as u16) << 8));
                        registers.sp.increment();
                    }
                    else if self.counter == 0 {
                        return true;
                    }

                    self.counter -= 1;
                    false
                }
            }

            #[cfg(test)]
            mod [<pop_ $register>] {
                use super::*;

                reusable_testing_macro!($opcode, [<Pop $register_upper>]);

                #[test]
                fn pop_low_on_tick_2() {
                    let mut registers = Registers::new(0, 0, 0, 0, 0xC000, 0xDFFE);
                    let mut alu = ALU::new(registers.f.clone());
                    let memory = Arc::new(Mutex::new(MemoryController::new()));
                    memory.lock().set(0xDFFE, 0x34);

                    let mut instruction = [<Pop $register_upper>] { counter: 2 };

                    let result = instruction.act(&mut registers, &mut alu, memory.clone(), &mut false, &mut false);

                    assert_eq!(false, result);
                    assert_eq!(0x0034, registers.$register.get_value());
                    assert_eq!(0xDFFF, registers.sp.get_value());
                }

                #[test]
                fn pop_high_on_tick_1() {
                    let mut registers = Registers::new(0, 0, 0, 0, 0xC000, 0xDFFF);
                    let mut alu = ALU::new(registers.f.clone());
                    let memory = Arc::new(Mutex::new(MemoryController::new()));
                    registers.$register.set_value(0x0034);
                    memory.lock().set(0xDFFF, 0x12);

                    let mut instruction = [<Pop $register_upper>] { counter: 1 };

                    let result = instruction.act(&mut registers, &mut alu, memory.clone(), &mut false, &mut false);

                    assert_eq!(false, result);
                    assert_eq!(0x1234, registers.$register.get_value());
                    assert_eq!(0xE000, registers.sp.get_value());
                }

                #[test]
                fn update_pc_on_tick_0_and_get_next_instruction() {
                    let mut registers = Registers::new(0, 0, 0, 0, 0xC000, 0xE000);
                    let mut alu = ALU::new(registers.f.clone());
                    let memory = Arc::new(Mutex::new(MemoryController::new()));

                    let mut instruction = [<Pop $register_upper>] { counter: 0 };

                    let result = instruction.act(&mut registers, &mut alu, memory.clone(), &mut false, &mut false);

                    assert_eq!(true, result);
                    assert_eq!(0xE000, registers.sp.get_value());
                }
            }
        }
    };
}


pop_rr!(0xC1, bc, BC);
pop_rr!(0xD1, de, DE);
pop_rr!(0xE1, hl, HL);
pop_rr!(0xF1, af, AF);


#[macro_export] macro_rules! pop_rr_decode_instruction {
    ($opcode:expr) => {
        return_if_is_instruction!(PopBC, $opcode);   //0xC5
        return_if_is_instruction!(PopDE, $opcode);   //0xD5
        return_if_is_instruction!(PopHL, $opcode);   //0xE5
        return_if_is_instruction!(PopAF, $opcode);   //0xF5
    }
}
