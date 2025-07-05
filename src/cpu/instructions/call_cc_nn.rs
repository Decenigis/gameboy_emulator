use crate::cpu::alu::ALU;
use crate::cpu::instructions::Instruction;
use crate::cpu::register::Register;
use crate::cpu::registers::Registers;
use crate::memory::{MemoryController, MemoryTrait};
use parking_lot::Mutex;
use paste::paste;
use std::sync::Arc;

macro_rules! call_cc_nn {
    ($opcode:literal, $suffix_lower:ident, $suffix:ident, $flag:ident, $wants_set:expr) => {
        paste!{
            pub struct [<Call $suffix Nn>] {
                counter: u8,
                address: u16
            }

            impl Instruction for [<Call $suffix Nn>] {

                #[inline]
                fn from_opcode(opcode: &u8) -> Option<Box<dyn Instruction>> {
                    if *opcode == $opcode {
                        return Some(Box::new([<Call $suffix Nn>] { counter: 2, address: 0 }))
                    }
                    None
                }

                fn get_opcode(&self) -> u8 {
                    $opcode
                }

                fn act(&mut self, registers: &mut Registers, _alu: &mut ALU, memory_controller: Arc<Mutex<MemoryController>>, _enable_interrupts: &mut bool, _is_halted: &mut bool) -> bool {
                    if self.counter == 2 {
                        self.address = memory_controller.lock().get(registers.pc.get_value()) as u16;
                        self.address = self.address | ((memory_controller.lock().get(registers.pc.get_value() + 1) as u16) << 8);

                        registers.pc.increment();
                        registers.pc.increment();
                    }

                    if registers.f.borrow().get_bit(ALU::$flag) == $wants_set {
                        if self.counter == 1 {
                            registers.sp.decrement();
                            memory_controller.lock().set(registers.sp.get_value(), ((registers.pc.get_value() & 0xFF00) >> 8) as u8);
                            registers.sp.decrement();
                            memory_controller.lock().set(registers.sp.get_value(), (registers.pc.get_value() & 0xFF) as u8);
                        }
                        else if self.counter == 0 {
                            registers.pc.set_value(self.address);
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
            mod [<call_ $suffix_lower _nn_tests>] {
                use super::*;

                reusable_testing_macro!($opcode, [<Call $suffix Nn>]);

                #[test]
                fn load_address_on_tick_2_if_condition_met() {
                    let mut registers = Registers::new(0, 0, 0, 0, 0xC000, 0xE000);
                    let mut alu = ALU::new(registers.f.clone());
                    let memory = Arc::new(Mutex::new(MemoryController::new()));

                    registers.f.borrow_mut().set_bit(ALU::$flag, $wants_set);

                    memory.lock().set(0xC000, 0x12);
                    memory.lock().set(0xC001, 0x34);

                    let mut instruction = [<Call $suffix Nn>] { counter: 2, address: 0 };

                    let result = instruction.act(&mut registers, &mut alu, memory.clone(), &mut false, &mut false);

                    assert_eq!(false, result);
                    assert_eq!(0x3412, instruction.address);
                    assert_eq!(0xC002, registers.pc.get_value());
                }

                #[test]
                fn increase_pc_if_condition_not_met() {
                    let mut registers = Registers::new(0, 0, 0, 0, 0xC000, 0xE000);
                    let mut alu = ALU::new(registers.f.clone());
                    let memory = Arc::new(Mutex::new(MemoryController::new()));

                    registers.f.borrow_mut().set_bit(ALU::$flag, !$wants_set);

                    let mut instruction = [<Call $suffix Nn>] { counter: 2, address: 0 };

                    let result = instruction.act(&mut registers, &mut alu, memory.clone(), &mut false, &mut false);

                    assert_eq!(false, result);
                    assert_eq!(0xC002, registers.pc.get_value());
                }

                #[test]
                fn saves_address_to_stack_on_tick_1_if_condition_met() {
                    let mut registers = Registers::new(0, 0, 0, 0, 0x1234, 0xE000);
                    let mut alu = ALU::new(registers.f.clone());
                    let memory = Arc::new(Mutex::new(MemoryController::new()));

                    registers.f.borrow_mut().set_bit(ALU::$flag, $wants_set);

                    let mut instruction = [<Call $suffix Nn>] { counter: 1, address: 0 };

                    let result = instruction.act(&mut registers, &mut alu, memory.clone(), &mut false, &mut false);

                    assert_eq!(false, result);
                    assert_eq!(0xDFFE, registers.sp.get_value());
                    assert_eq!(0x34, memory.lock().get(registers.sp.get_value()));
                    assert_eq!(0x12, memory.lock().get(registers.sp.get_value() + 1));
                }

                #[test]
                fn do_not_save_address_to_stack_on_tick_1_if_condition_not_met() {
                    let mut registers = Registers::new(0, 0, 0, 0, 0x1234, 0xE000);
                    let mut alu = ALU::new(registers.f.clone());
                    let memory = Arc::new(Mutex::new(MemoryController::new()));

                    registers.f.borrow_mut().set_bit(ALU::$flag, !$wants_set);

                    let mut instruction = [<Call $suffix Nn>] { counter: 1, address: 0 };

                    let result = instruction.act(&mut registers, &mut alu, memory.clone(), &mut false, &mut false);

                    assert_eq!(false, result);
                    assert_eq!(0xE000, registers.sp.get_value());
                    assert_eq!(0x00, memory.lock().get(registers.sp.get_value() - 1));
                    assert_eq!(0x00, memory.lock().get(registers.sp.get_value() - 2));
                }

                #[test]
                fn update_pc_on_tick_0_and_get_next_instruction() {
                    let mut registers = Registers::new(0, 0, 0, 0, 0xC000, 0);
                    let mut alu = ALU::new(registers.f.clone());
                    let memory = Arc::new(Mutex::new(MemoryController::new()));

                    registers.f.borrow_mut().set_bit(ALU::$flag, $wants_set);

                    let mut instruction = [<Call $suffix Nn>] { counter: 0, address: 0x1234 };

                    let result = instruction.act(&mut registers, &mut alu, memory.clone(), &mut false, &mut false);

                    assert_eq!(true, result);

                    assert_eq!(0x1234, registers.pc.get_value());
                }

                #[test]
                fn do_not_update_pc_on_tick_0_if_condition_not_met() {
                    let mut registers = Registers::new(0, 0, 0, 0, 0xC000, 0);
                    let mut alu = ALU::new(registers.f.clone());
                    let memory = Arc::new(Mutex::new(MemoryController::new()));

                    registers.f.borrow_mut().set_bit(ALU::$flag, !$wants_set);

                    let mut instruction = [<Call $suffix Nn>] { counter: 0, address: 0x1234 };

                    let result = instruction.act(&mut registers, &mut alu, memory.clone(), &mut false, &mut false);

                    assert_eq!(true, result);

                    assert_eq!(0xC000, registers.pc.get_value());
                }
            }
        }
    }
}


call_cc_nn!(0xC4, nz, Nz, ZERO_FLAG, false);
call_cc_nn!(0xCC, z, Z, ZERO_FLAG, true);
call_cc_nn!(0xD4, nc, Nc, CARRY_FLAG, false);
call_cc_nn!(0xDC, c, C, CARRY_FLAG, false);



#[macro_export] macro_rules! call_cc_nn_decode_instruction {
    ($opcode:expr) => {
        return_if_is_instruction!(CallNzNn, $opcode);  //0xC4
        return_if_is_instruction!(CallZNn, $opcode);   //0xCC
        return_if_is_instruction!(CallNcNn, $opcode);  //0xD4
        return_if_is_instruction!(CallCNn, $opcode);   //0xDC
    }
}
