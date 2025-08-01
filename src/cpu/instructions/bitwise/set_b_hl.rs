use crate::cpu::register8::Register8;
use crate::cpu::alu::ALU;
use crate::cpu::instructions::Instruction;
use crate::cpu::register::Register;
use crate::cpu::registers::Registers;
use crate::memory::{MemoryController, MemoryTrait};
use parking_lot::Mutex;
use paste::paste;
use std::sync::Arc;


macro_rules! set_b_hl {
    ($opcode:literal, $bit:literal) => {
        paste!{
            pub struct [<Set $bit Hl>] {
                counter: u8,
                value_register: Register8
            }

            impl Instruction for [<Set $bit Hl>] {
                #[inline]
                fn from_opcode(opcode: &u8) -> Option<Box<dyn Instruction>> {
                    if *opcode == $opcode {
                        return Some(Box::new([<Set $bit Hl>]{ counter: 3, value_register: Register8::new(0) }))
                    }
                    None
                }

                fn get_opcode(&self) -> u8 {
                    $opcode
                }

                fn act(&mut self, registers: &mut Registers, _alu: &mut ALU, memory_controller: Arc<Mutex<MemoryController>>, _enable_interrupts: &mut bool, _is_halted: &mut bool) -> bool {
                    if self.counter == 3 {
                        self.value_register.set_value(memory_controller.lock().get(registers.hl.get_value()));
                    }
                    else if self.counter == 2 {
                        self.value_register.set_bit($bit, true);
                    }
                    else if self.counter == 1 {
                        memory_controller.lock().set(registers.hl.get_value(), self.value_register.get_value());
                    }
                    else if self.counter == 0 {
                        return true;
                    }

                    self.counter -= 1;
                    false
                }
            }


            #[cfg(test)]
            mod [<set_ $bit _ hl_tests>] {
                use super::*;

                reusable_testing_macro!($opcode, [<Set $bit Hl>]);

                #[test]
                fn tests_bit_when_set_on_tick_3() {
                    let mut registers = Registers::new(0, 0, 0, 0xC000, 0xC000, 0);
                    let memory = Arc::new(Mutex::new(MemoryController::new()));
                    let mut alu = ALU::new(registers.f.clone());

                    memory.lock().set(0xC000, 0x12);

                    let mut instruction = [<Set $bit Hl>] { counter: 3, value_register: Register8::new(0) };

                    let result = instruction.act(&mut registers, &mut alu, memory.clone(), &mut false, &mut false);

                    assert_eq!(false, result);
                    assert_eq!(0x12, instruction.value_register.get_value());
                }

                #[test]
                fn set_bit_on_tick_2() {
                    let mut registers = Registers::new(0, 0, 0, 0xC000, 0xC000, 0);
                    let memory = Arc::new(Mutex::new(MemoryController::new()));
                    let mut alu = ALU::new(registers.f.clone());

                    let mut instruction = [<Set $bit Hl>] { counter: 2, value_register: Register8::new(0x00) };

                    let result = instruction.act(&mut registers, &mut alu, memory.clone(), &mut false, &mut false);

                    assert_eq!(false, result);
                    assert_eq!(true, instruction.value_register.get_bit($bit));
                }

                #[test]
                fn write_value_on_tick_1() {
                    let mut registers = Registers::new(0, 0, 0, 0xC000, 0xC000, 0);
                    let memory = Arc::new(Mutex::new(MemoryController::new()));
                    let mut alu = ALU::new(registers.f.clone());

                    let mut instruction = [<Set $bit Hl>] { counter: 1, value_register: Register8::new(0x12) };

                    let result = instruction.act(&mut registers, &mut alu, memory.clone(), &mut false, &mut false);

                    assert_eq!(false, result);
                    assert_eq!(0x12, memory.lock().get(registers.hl.get_value()));
                }

                #[test]
                fn get_next_instruction_on_tick_0() {
                    let mut registers = Registers::new(0, 0, 0, 0xC000, 0xC000, 0);
                    let memory = Arc::new(Mutex::new(MemoryController::new()));
                    let mut alu = ALU::new(registers.f.clone());

                    let mut instruction = [<Set $bit Hl>] { counter: 0, value_register: Register8::new(0x12) };

                    let result = instruction.act(&mut registers, &mut alu, memory.clone(), &mut false, &mut false);

                    assert_eq!(true, result);
                }
            }
        }
    }
}

set_b_hl!(0xC6, 0);
set_b_hl!(0xCE, 1);
set_b_hl!(0xD6, 2);
set_b_hl!(0xDE, 3);
set_b_hl!(0xE6, 4);
set_b_hl!(0xEE, 5);
set_b_hl!(0xF6, 6);
set_b_hl!(0xFE, 7);



#[macro_export] macro_rules! set_b_hl_decode_instruction {
    ($opcode:expr) => {
        return_if_is_instruction!(Set0Hl, $opcode); //0xC6
        return_if_is_instruction!(Set1Hl, $opcode); //0xCE
        return_if_is_instruction!(Set2Hl, $opcode); //0xD6
        return_if_is_instruction!(Set3Hl, $opcode); //0xDE
        return_if_is_instruction!(Set4Hl, $opcode); //0xE6
        return_if_is_instruction!(Set5Hl, $opcode); //0xEA
        return_if_is_instruction!(Set6Hl, $opcode); //0xF6
        return_if_is_instruction!(Set7Hl, $opcode); //0xFE
    }
}
