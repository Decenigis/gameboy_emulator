use crate::cpu::register8::Register8;
use crate::cpu::alu::ALU;
use crate::cpu::instructions::Instruction;
use crate::cpu::register::Register;
use crate::cpu::registers::Registers;
use crate::memory::{MemoryController, MemoryTrait};
use parking_lot::Mutex;
use paste::paste;
use std::sync::Arc;


macro_rules! bit_b_hl {
    ($opcode:literal, $bit:literal) => {
        paste!{
            pub struct [<Bit $bit Hl>] {
                counter: u8,
                value_register: Register8
            }

            impl Instruction for [<Bit $bit Hl>] {
                #[inline]
                fn from_opcode(opcode: &u8) -> Option<Box<dyn Instruction>> {
                    if *opcode == $opcode {
                        return Some(Box::new([<Bit $bit Hl>]{ counter: 3, value_register: Register8::new(0) }))
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
                    if self.counter == 2 {
                        let mut flags = registers.f.borrow_mut();

                        flags.set_bit(ALU::SUB_FLAG, false);
                        flags.set_bit(ALU::HALF_CARRY_FLAG, true);
                        flags.set_bit(ALU::ZERO_FLAG, !self.value_register.get_bit($bit));
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
            mod [<bit_ $bit _hl_tests>] {
                use super::*;

                reusable_testing_macro!($opcode, [<Bit $bit Hl>]);

                #[test]
                fn gets_byte_on_tick_3() {
                    let mut registers = Registers::new(0, 0, 0, 0xC000, 0xC000, 0);
                    let memory = Arc::new(Mutex::new(MemoryController::new()));
                    let mut alu = ALU::new(registers.f.clone());

                    memory.lock().set(0xC000, 0x12);

                    let mut instruction = [<Bit $bit Hl>] { counter: 3, value_register: Register8::new(0) };

                    let result = instruction.act(&mut registers, &mut alu, memory.clone(), &mut false, &mut false);

                    assert_eq!(0x12, instruction.value_register.get_value());
                    assert_eq!(false, result);
                }

                #[test]
                fn tests_bit_when_set_on_tick_3() {
                    let mut registers = Registers::new(0, 0, 0, 0, 0xC000, 0);
                    let memory = Arc::new(Mutex::new(MemoryController::new()));
                    let mut alu = ALU::new(registers.f.clone());

                    let mut instruction = [<Bit $bit Hl>] { counter: 2, value_register: Register8::new(0xFF) };

                    let result = instruction.act(&mut registers, &mut alu, memory.clone(), &mut false, &mut false);

                    assert_eq!(false, result);
                    assert_eq!(false, registers.f.borrow().get_bit(ALU::SUB_FLAG));
                    assert_eq!(true, registers.f.borrow().get_bit(ALU::HALF_CARRY_FLAG));
                    assert_eq!(false, registers.f.borrow().get_bit(ALU::ZERO_FLAG));
                }

                #[test]
                fn tests_bit_when_unset_on_tick_2() {
                    let mut registers = Registers::new(0, 0, 0, 0, 0xC000, 0);
                    let memory = Arc::new(Mutex::new(MemoryController::new()));
                    let mut alu = ALU::new(registers.f.clone());

                    let mut instruction = [<Bit $bit Hl>] { counter: 2, value_register: Register8::new(0) };

                    let result = instruction.act(&mut registers, &mut alu, memory.clone(), &mut false, &mut false);

                    assert_eq!(false, result);
                    assert_eq!(false, registers.f.borrow().get_bit(ALU::SUB_FLAG));
                    assert_eq!(true, registers.f.borrow().get_bit(ALU::HALF_CARRY_FLAG));
                    assert_eq!(true, registers.f.borrow().get_bit(ALU::ZERO_FLAG));
                }

                #[test]
                fn sets_byte_on_tick_1() {
                    let mut registers = Registers::new(0, 0, 0, 0xC000, 0xC000, 0);
                    let memory = Arc::new(Mutex::new(MemoryController::new()));
                    let mut alu = ALU::new(registers.f.clone());

                    let mut instruction = [<Bit $bit Hl>] { counter: 1, value_register: Register8::new(0x12)};

                    let result = instruction.act(&mut registers, &mut alu, memory.clone(), &mut false, &mut false);

                    assert_eq!(0x12, memory.lock().get(registers.hl.get_value()));
                    assert_eq!(false, result);
                }

                #[test]
                fn returns_on_tick_0() {
                    let mut registers = Registers::new(0, 0, 0, 0, 0xC000, 0);
                    let memory = Arc::new(Mutex::new(MemoryController::new()));
                    let mut alu = ALU::new(registers.f.clone());

                    let mut instruction = [<Bit $bit Hl>] { counter: 0, value_register: Register8::new(0) };

                    let result = instruction.act(&mut registers, &mut alu, memory.clone(), &mut false, &mut false);

                    assert_eq!(true, result);
                }
            }
        }
    }
}

bit_b_hl!(0x46, 0);
bit_b_hl!(0x4E, 1);
bit_b_hl!(0x56, 2);
bit_b_hl!(0x5E, 3);
bit_b_hl!(0x66, 4);
bit_b_hl!(0x6E, 5);
bit_b_hl!(0x76, 6);
bit_b_hl!(0x7E, 7);


#[macro_export] macro_rules! bit_b_hl_decode_instruction {
    ($opcode:expr) => {
        return_if_is_instruction!(Bit0Hl, $opcode);
        return_if_is_instruction!(Bit1Hl, $opcode);
        return_if_is_instruction!(Bit2Hl, $opcode);
        return_if_is_instruction!(Bit3Hl, $opcode);
        return_if_is_instruction!(Bit4Hl, $opcode);
        return_if_is_instruction!(Bit5Hl, $opcode);
        return_if_is_instruction!(Bit6Hl, $opcode);
        return_if_is_instruction!(Bit7Hl, $opcode);
    }
}
