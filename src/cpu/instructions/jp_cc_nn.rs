use paste::paste;
use crate::cpu::alu::ALU;
use crate::cpu::instructions::Instruction;
use crate::cpu::register::Register;
use crate::cpu::registers::Registers;
use crate::memory::{MemoryController, MemoryTrait};
use parking_lot::Mutex;
use std::sync::Arc;

macro_rules! jp_cc_nn {
    ($opcode:literal, $suffix_lower:ident, $suffix:ident, $flag:ident, $wants_set:expr) => {
        paste!{
            pub struct [<Jp $suffix Nn>]  {
                counter: u8,
                address: u16
            }

            impl Instruction for [<Jp $suffix Nn>] {

                #[inline]
                fn from_opcode(opcode: &u8) -> Option<Box<dyn Instruction>> {
                    if *opcode == $opcode {
                        return Some(Box::new([<Jp $suffix Nn>] { counter: 2, address: 0 }))
                    }
                    None
                }

                fn get_opcode(&self) -> u8 {
                    $opcode
                }

                fn act(&mut self, registers: &mut Registers, _alu: &mut ALU, memory_controller: Arc<Mutex<MemoryController>>, _enable_interrupts: &mut bool, _is_halted: &mut bool) -> bool {
                    if self.counter == 2 {
                        self.address = memory_controller.lock().get(registers.pc.get_value()) as u16;
                        registers.pc.increment();
                    }
                    else if self.counter == 1 {
                        self.address = self.address | ((memory_controller.lock().get(registers.pc.get_value()) as u16) << 8);
                        registers.pc.increment();
                    }
                    else if self.counter == 0 {
                        if registers.f.borrow().get_bit(ALU::$flag) == $wants_set {
                            registers.pc.set_value(self.address);
                        }

                        return true;
                    }

                    self.counter -= 1;
                    false
                }
            }



            #[cfg(test)]
            mod [<jp_ $suffix_lower _nn>]  {
                use super::*;

                reusable_testing_macro!($opcode, [<Jp $suffix Nn>]);

            #[test]
            fn load_low_byte_on_tick_2() {
                let mut registers = Registers::new(0, 0, 0, 0, 0xC000, 0);
                let mut alu = ALU::new(registers.f.clone());
                let memory = Arc::new(Mutex::new(MemoryController::new()));

                memory.lock().set(0xC000, 0x12);

                let mut instruction = [<Jp $suffix Nn>] { counter: 2, address: 0 };

                let result = instruction.act(&mut registers, &mut alu, memory.clone(),&mut false, &mut false);

                assert_eq!(false, result);

                assert_eq!(0x0012, instruction.address);
            }

            #[test]
            fn load_high_byte_on_tick_1() {
                let mut registers = Registers::new(0, 0, 0, 0, 0xC000, 0);
                let mut alu = ALU::new(registers.f.clone());
                let memory = Arc::new(Mutex::new(MemoryController::new()));

                memory.lock().set(0xC000, 0x12);

                let mut instruction = [<Jp $suffix Nn>] { counter: 1, address: 0 };

                let result = instruction.act(&mut registers, &mut alu, memory.clone(),&mut false, &mut false);

                assert_eq!(false, result);

                assert_eq!(0x1200, instruction.address);
            }


                #[test]
                fn update_pc_on_tick_0_if_nz_and_get_next_instruction() {
                    let mut registers = Registers::new(0, 0, 0, 0, 0xC000, 0);
                    let mut alu = ALU::new(registers.f.clone());
                    let memory = Arc::new(Mutex::new(MemoryController::new()));

                    registers.f.borrow_mut().set_bit(ALU::$flag, $wants_set);

                    let mut instruction = [<Jp $suffix Nn>] { counter: 0, address: 0x1234 };

                    let result = instruction.act(&mut registers, &mut alu, memory.clone(),&mut false, &mut false);

                    assert_eq!(true, result);

                    assert_eq!(0x1234, registers.pc.get_value());
                }


                #[test]
                fn do_not_change_pc_on_tick_0_if_z_and_get_next_instruction() {
                    let mut registers = Registers::new(0, 0, 0, 0, 0xC000, 0);
                    let mut alu = ALU::new(registers.f.clone());
                    let memory = Arc::new(Mutex::new(MemoryController::new()));

                    registers.f.borrow_mut().set_bit(ALU::$flag, !$wants_set);

                    let mut instruction = [<Jp $suffix Nn>] { counter: 0, address: 0x1234 };

                    let result = instruction.act(&mut registers, &mut alu, memory.clone(),&mut false, &mut false);

                    assert_eq!(true, result);

                    assert_eq!(0xC000, registers.pc.get_value());
                }
            }
        }
    }
}

jp_cc_nn!(0xC2, nz, Nz, ZERO_FLAG, false);
jp_cc_nn!(0xCA, z, Z, ZERO_FLAG, true);
jp_cc_nn!(0xD2, nc, Nc, CARRY_FLAG, false);
jp_cc_nn!(0xDA, c, C, CARRY_FLAG, true);



#[macro_export] macro_rules! jp_cc_nn_decode_instruction {
    ($opcode:expr) => {
        return_if_is_instruction!(JpNzNn, $opcode);  //0xC4
        return_if_is_instruction!(JpZNn, $opcode);   //0xCC
        return_if_is_instruction!(JpNcNn, $opcode);  //0xD4
        return_if_is_instruction!(JpCNn, $opcode);   //0xDC
    }
}
