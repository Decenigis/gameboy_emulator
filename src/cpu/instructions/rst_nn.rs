use crate::cpu::alu::ALU;
use crate::cpu::instructions::Instruction;
use crate::cpu::registers::Registers;
use crate::cpu::register::Register;
use crate::memory::{MemoryController, MemoryTrait};
use parking_lot::Mutex;
use paste::paste;
use std::sync::Arc;


macro_rules! rst_nn {
    ($opcode:literal, $address_name:literal, $address_hex:literal) => {
        paste!{
            pub struct [<Rst $address_name>]  {
                counter: u8
            }

            impl Instruction for [<Rst $address_name>] {
                #[inline]
                fn from_opcode(opcode: &u8) -> Option<Box<dyn Instruction>> {
                    if *opcode == $opcode {
                        return Some(Box::new([<Rst $address_name>]  { counter: 7 }))
                    }
                    None
                }


                fn get_opcode(&self) -> u8 {
                    $opcode
                }

                fn act(&mut self, registers: &mut Registers, alu: &mut ALU, memory_controller: Arc<Mutex<MemoryController>>, _enable_interrupts: &mut bool, _is_halted: &mut bool) -> bool {
                    if self.counter == 7 {
                        registers.sp.decrement();
                        memory_controller.lock().set(registers.sp.get_value(), ((registers.pc.get_value() & 0xFF00) >> 8) as u8);
                    }
                    else if self.counter == 6 {
                        registers.sp.decrement();
                        memory_controller.lock().set(registers.sp.get_value(), (registers.pc.get_value() & 0xFF) as u8);
                    }
                    else if self.counter == 5 {
                        registers.pc.set_value($address_hex);
                    }
                    else if self.counter == 0 {
                        return true;
                    }

                    self.counter -= 1;
                    false
                }
            }

            #[cfg(test)]
            mod [<rst_ $address_name _tests>] {
                use super::*;

                reusable_testing_macro!($opcode, [<Rst $address_name>] );

                #[test]
                fn get_low_return_byte_on_clock_7() {
                    let mut registers = Registers::new(0, 0, 0, 0, 0x1234, 0xE000);
                    let mut alu = ALU::new(registers.f.clone());
                    let memory = Arc::new(Mutex::new(MemoryController::new()));

                    memory.lock().set(0xC000, 0x12);

                    let mut instruction = [<Rst $address_name>] { counter: 7 };

                    let result = instruction.act(&mut registers, &mut alu, memory.clone() ,&mut false, &mut false);

                    assert_eq!(false, result);

                    assert_eq!(0xDFFF, registers.sp.get_value());
                    assert_eq!(0x12, memory.lock().get(registers.sp.get_value()));
                }

                #[test]
                fn get_low_return_byte_on_clock_6() {
                    let mut registers = Registers::new(0, 0, 0, 0, 0x1234, 0xDFFF);
                    let mut alu = ALU::new(registers.f.clone());
                    let memory = Arc::new(Mutex::new(MemoryController::new()));

                    memory.lock().set(0xC000, 0x12);

                    let mut instruction = [<Rst $address_name>] { counter: 6 };

                    let result = instruction.act(&mut registers, &mut alu, memory.clone() ,&mut false, &mut false);

                    assert_eq!(false, result);

                    assert_eq!(0xDFFE, registers.sp.get_value());
                    assert_eq!(0x34, memory.lock().get(registers.sp.get_value()));
                }

                #[test]
                fn set_pc_on_tick_5() {
                    let mut registers = Registers::new(0, 0, 0, 0, 0x1234, 0xDFFE);
                    let mut alu = ALU::new(registers.f.clone());
                    let memory = Arc::new(Mutex::new(MemoryController::new()));

                    let mut instruction = [<Rst $address_name>] { counter: 5 };

                    let result = instruction.act(&mut registers, &mut alu, memory.clone() ,&mut false, &mut false);

                    assert_eq!(false, result);
                    assert_eq!(0xDFFE, registers.sp.get_value());
                    assert_eq!($address_name, registers.pc.get_value());
                }

                #[test]
                fn get_next_instruction_on_tick_0 () {
                    let mut registers = Registers::new(0, 0, 0, 0, 0x1234, 0xDFFE);
                    let mut alu = ALU::new(registers.f.clone());
                    let memory = Arc::new(Mutex::new(MemoryController::new()));

                    let mut instruction = [<Rst $address_name>] { counter: 0 };

                    let result = instruction.act(&mut registers, &mut alu, memory.clone() ,&mut false, &mut false);

                    assert_eq!(true, result);
                }
            }
        }
    }
}


rst_nn!(0xC7, 00, 0x00);
rst_nn!(0xCF, 08, 0x08);
rst_nn!(0xD7, 10, 0x10);
rst_nn!(0xDF, 18, 0x18);
rst_nn!(0xE7, 20, 0x20);
rst_nn!(0xEF, 28, 0x28);
rst_nn!(0xF7, 30, 0x30);
rst_nn!(0xFF, 38, 0x38);


#[macro_export] macro_rules! rst_nn_decode_instruction {
    ($opcode:expr) => {
        return_if_is_instruction!(Rst00, $opcode);   //0xC7
        return_if_is_instruction!(Rst08, $opcode);   //0xCF
        return_if_is_instruction!(Rst10, $opcode);   //0xD7
        return_if_is_instruction!(Rst18, $opcode);   //0xDF
        return_if_is_instruction!(Rst20, $opcode);   //0xE7
        return_if_is_instruction!(Rst28, $opcode);   //0xEF
        return_if_is_instruction!(Rst30, $opcode);   //0xF7
        return_if_is_instruction!(Rst38, $opcode);   //0xFF
    }
}
