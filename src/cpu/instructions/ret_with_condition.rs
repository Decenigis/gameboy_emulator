use paste::paste;
use std::sync::Arc;
use parking_lot::Mutex;
use crate::cpu::alu::ALU;
use crate::cpu::instructions::Instruction;
use crate::cpu::register::Register;
use crate::cpu::registers::Registers;
use crate::memory::{MemoryController, MemoryTrait};



macro_rules! ret_with_condition {
    ($opcode:literal, $suffix_lower:ident, $suffix:ident, $flag:ident, $wants_set:expr) => {
        paste!{
            pub struct [<Ret $suffix>] {
                counter: u8,
                address_low_byte: u8,
            }

            impl Instruction for [<Ret $suffix>] {
                #[inline]
                fn from_opcode(opcode: &u8) -> Option<Box<dyn Instruction>> {
                    if *opcode == $opcode {
                        return Some(Box::new([<Ret $suffix>] { counter: 1, address_low_byte: 0 }))
                    }
                    None
                }

                fn get_opcode(&self) -> u8 {
                    $opcode
                }

                fn act(&mut self, registers: &mut Registers, _alu: &mut ALU, memory_controller: Arc<Mutex<MemoryController>>, _enable_interrupts: &mut bool) -> bool {
                    if registers.f.borrow().get_bit(ALU::$flag) == $wants_set {
                        if self.counter == 1 {
                            self.address_low_byte = memory_controller.lock().get(registers.sp.get_value());
                            registers.sp.increment();
                        }
                        else if self.counter == 0 {
                            let address_high_byte = memory_controller.lock().get(registers.sp.get_value());
                            registers.sp.increment();

                            registers.pc.set_value((address_high_byte as u16) << 8 | (self.address_low_byte as u16));

                            return true;
                        }
                    }
                    else if self.counter == 0 {
                        return true;
                    }

                    self.counter -= 1;
                    false
                }
            }



            #[cfg(test)]
            mod [<ret_ $suffix_lower _tests>] {
                use super::*;

                reusable_testing_macro!($opcode, [<Ret $suffix>]);

                #[test]
                fn get_low_return_byte_on_clock_1_if_condition_met() {
                    let mut registers = Registers::new(0, 0, 0, 0, 0x0000, 0xC000);
                    registers.f.borrow_mut().set_bit(ALU::$flag, $wants_set);

                    let mut alu = ALU::new(registers.f.clone());
                    let memory = Arc::new(Mutex::new(MemoryController::new()));

                    memory.lock().set(0xC000, 0x12);

                    let mut instruction = [<Ret $suffix>] { counter: 1, address_low_byte: 0 };

                    let result = instruction.act(&mut registers, &mut alu, memory,&mut false);

                    assert_eq!(false, result);
                    assert_eq!(0x12, instruction.address_low_byte);
                    assert_eq!(0xC001, registers.sp.get_value());
                }

                #[test]
                fn do_nothing_on_clock_1_if_condition_not_met() {
                    let mut registers = Registers::new(0, 0, 0, 0, 0x0000, 0xC000);
                    registers.f.borrow_mut().set_bit(ALU::$flag, !$wants_set);

                    let mut alu = ALU::new(registers.f.clone());
                    let memory = Arc::new(Mutex::new(MemoryController::new()));

                    memory.lock().set(0xC000, 0x12);

                    let mut instruction = [<Ret $suffix>] { counter: 1, address_low_byte: 0 };

                    let result = instruction.act(&mut registers, &mut alu, memory,&mut false);

                    assert_eq!(false, result);
                    assert_eq!(0x00, instruction.address_low_byte);
                    assert_eq!(0xC000, registers.sp.get_value());
                }

                #[test]
                fn set_pc_on_clock_0_and_get_next_instruction_if_condition_met() {
                    let mut registers = Registers::new(0, 0, 0, 0, 0x0000, 0xC001);
                    registers.f.borrow_mut().set_bit(ALU::$flag, $wants_set);

                    let mut alu = ALU::new(registers.f.clone());
                    let memory = Arc::new(Mutex::new(MemoryController::new()));

                    memory.lock().set(0xC001, 0x34);

                    let mut instruction = [<Ret $suffix>] { counter: 0, address_low_byte: 0x12 };

                    let result = instruction.act(&mut registers, &mut alu, memory,&mut false);

                    assert_eq!(true, result);
                    assert_eq!(0x3412, registers.pc.get_value());
                    assert_eq!(0xC002, registers.sp.get_value());
                }

                #[test]
                fn clock_0_and_get_next_instruction_if_condition_not_met() {
                    let mut registers = Registers::new(0, 0, 0, 0, 0x0000, 0xC000);
                    registers.f.borrow_mut().set_bit(ALU::$flag, !$wants_set);

                    let mut alu = ALU::new(registers.f.clone());
                    let memory = Arc::new(Mutex::new(MemoryController::new()));

                    memory.lock().set(0xC000, 0x34);

                    let mut instruction = [<Ret $suffix>] { counter: 0, address_low_byte: 0x12 };

                    let result = instruction.act(&mut registers, &mut alu, memory,&mut false);

                    assert_eq!(true, result);
                    assert_eq!(0x0000, registers.pc.get_value());
                    assert_eq!(0xC000, registers.sp.get_value());
                }
            }
        }
    }
}


ret_with_condition!(0xC0, nz, NZ, ZERO_FLAG, false);
ret_with_condition!(0xC8, z, Z, ZERO_FLAG, true);
ret_with_condition!(0xD0, nc, NC, CARRY_FLAG, false);
ret_with_condition!(0xD8, c, C, CARRY_FLAG, true);


#[macro_export] macro_rules! ret_with_condition_decode_instruction {
    ($opcode:expr) => {
        return_if_is_instruction!(RetNZ, $opcode);  //0xC0
        return_if_is_instruction!(RetZ, $opcode);   //0xC8
        return_if_is_instruction!(RetNC, $opcode);  //0xD0
        return_if_is_instruction!(RetC, $opcode);   //0xD8
    }
}
