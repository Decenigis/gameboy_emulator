use crate::cpu::register8::Register8;
use crate::cpu::alu::ALU;
use crate::cpu::instructions::Instruction;
use crate::cpu::register::Register;
use crate::cpu::registers::Registers;
use crate::memory::{MemoryController, MemoryTrait};
use parking_lot::Mutex;
use paste::paste;
use std::sync::Arc;


macro_rules! res_b_hl {
    ($opcode:literal, $bit:literal) => {
        paste!{
            pub struct [<Res $bit Hl>] {
                counter: u8,
                value_register: Register8
            }

            impl Instruction for [<Res $bit Hl>] {
                #[inline]
                fn from_opcode(opcode: &u8) -> Option<Box<dyn Instruction>> {
                    if *opcode == $opcode {
                        return Some(Box::new([<Res $bit Hl>]{ counter: 3, value_register: Register8::new(0) }))
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
                        self.value_register.set_bit($bit, false);
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
            mod [<res_ $bit _ hl_tests>] {
                use super::*;

                reusable_testing_macro!($opcode, [<Res $bit Hl>]);

                #[test]
                fn tests_bit_when_set_on_tick_3() {
                    let mut registers = Registers::new(0, 0, 0, 0xC000, 0xC000, 0);
                    let memory = Arc::new(Mutex::new(MemoryController::new()));
                    let mut alu = ALU::new(registers.f.clone());

                    memory.lock().set(0xC000, 0x12);

                    let mut instruction = [<Res $bit Hl>] { counter: 3, value_register: Register8::new(0) };

                    let result = instruction.act(&mut registers, &mut alu, memory.clone(), &mut false, &mut false);

                    assert_eq!(false, result);
                    assert_eq!(0x12, instruction.value_register.get_value());
                }

                #[test]
                fn reset_bit_on_tick_2() {
                    let mut registers = Registers::new(0, 0, 0, 0xC000, 0xC000, 0);
                    let memory = Arc::new(Mutex::new(MemoryController::new()));
                    let mut alu = ALU::new(registers.f.clone());

                    let mut instruction = [<Res $bit Hl>] { counter: 2, value_register: Register8::new(0xFF) };

                    let result = instruction.act(&mut registers, &mut alu, memory.clone(), &mut false, &mut false);

                    assert_eq!(false, result);
                    assert_eq!(false, instruction.value_register.get_bit($bit));
                }

                #[test]
                fn reset_bit_on_tick_1() {
                    let mut registers = Registers::new(0, 0, 0, 0xC000, 0xC000, 0);
                    let memory = Arc::new(Mutex::new(MemoryController::new()));
                    let mut alu = ALU::new(registers.f.clone());

                    let mut instruction = [<Res $bit Hl>] { counter: 1, value_register: Register8::new(0x12) };

                    let result = instruction.act(&mut registers, &mut alu, memory.clone(), &mut false, &mut false);

                    assert_eq!(false, result);
                    assert_eq!(0x12, memory.lock().get(registers.hl.get_value()));
                }

                #[test]
                fn get_next_instruction_on_tick_0() {
                    let mut registers = Registers::new(0, 0, 0, 0xC000, 0xC000, 0);
                    let memory = Arc::new(Mutex::new(MemoryController::new()));
                    let mut alu = ALU::new(registers.f.clone());

                    let mut instruction = [<Res $bit Hl>] { counter: 0, value_register: Register8::new(0x12) };

                    let result = instruction.act(&mut registers, &mut alu, memory.clone(), &mut false, &mut false);

                    assert_eq!(true, result);
                }
            }
        }
    }
}

res_b_hl!(0x86, 0);
res_b_hl!(0x8E, 1);
res_b_hl!(0x96, 2);
res_b_hl!(0x9E, 3);
res_b_hl!(0xA6, 4);
res_b_hl!(0xAA, 5);
res_b_hl!(0xB6, 6);
res_b_hl!(0xBE, 7);



#[macro_export] macro_rules! res_b_hl_decode_instruction {
    ($opcode:expr) => {
        return_if_is_instruction!(Res0Hl, $opcode); //0x86
        return_if_is_instruction!(Res1Hl, $opcode); //0x8E
        return_if_is_instruction!(Res2Hl, $opcode); //0x96
        return_if_is_instruction!(Res3Hl, $opcode); //0x9E
        return_if_is_instruction!(Res4Hl, $opcode); //0xA6
        return_if_is_instruction!(Res5Hl, $opcode); //0xAA
        return_if_is_instruction!(Res6Hl, $opcode); //0xB6
        return_if_is_instruction!(Res7Hl, $opcode); //0xBE
    }
}
