use paste::paste;
use std::sync::Arc;
use parking_lot::Mutex;
use crate::cpu::alu::ALU;
use crate::cpu::instructions::Instruction;
use crate::cpu::register::Register;
use crate::cpu::registers::Registers;
use crate::memory::{MemoryController, MemoryTrait};

macro_rules! ld_rr_nn {
    ($opcode:literal, $register:ident, $register_upper:ident) => {
        paste!{
            pub struct [<Ld $register_upper Nn>] {
                counter: u8,
                value: u16
            }

            impl Instruction for [<Ld $register_upper Nn>]  {

                #[inline]
                fn from_opcode(opcode: &u8) -> Option<Box<dyn Instruction>> {
                    if *opcode == $opcode {
                        return Some(Box::new([<Ld $register_upper Nn>]  { counter: 2, value: 0 }))
                    }
                    None
                }

                fn get_opcode(&self) -> u8 {
                    $opcode
                }

                fn act(&mut self, registers: &mut Registers, _alu: &mut ALU, memory_controller: Arc<Mutex<MemoryController>>, _enable_interrupts: &mut bool) -> bool {
                    if self.counter == 2 {
                        self.value = memory_controller.lock().get(registers.pc.get_value()) as u16;
                        registers.pc.increment();
                    }
                    else if self.counter == 1 {
                        self.value = self.value | ((memory_controller.lock().get(registers.pc.get_value()) as u16) << 8);
                        registers.pc.increment();
                    }
                    else if self.counter == 0 {
                        registers.$register.set_value(self.value);
                        return true;
                    }

                    self.counter -= 1;
                    false
                }
            }

            #[cfg(test)]
            mod [<ld_ $register _nn_tests>] {
                use super::*;

                reusable_testing_macro!($opcode, [<Ld $register_upper Nn>] );

                #[test]
                fn load_low_byte_on_tick_2() {
                    let mut registers = Registers::new(0, 0, 0, 0, 0xC000, 0);
                    let mut alu = ALU::new(registers.f.clone());
                    let memory = Arc::new(Mutex::new(MemoryController::new()));

                    memory.lock().set(0xC000, 0x12);

                    let mut instruction = [<Ld $register_upper Nn>]  { counter: 2, value: 0 };

                    let result = instruction.act(&mut registers, &mut alu, memory.clone(),&mut false);

                    assert_eq!(false, result);

                    assert_eq!(0x0012, instruction.value);
                }

                #[test]
                fn load_high_byte_on_tick_1() {
                    let mut registers = Registers::new(0, 0, 0, 0, 0xC000, 0);
                    let mut alu = ALU::new(registers.f.clone());
                    let memory = Arc::new(Mutex::new(MemoryController::new()));

                    memory.lock().set(0xC000, 0x12);

                    let mut instruction = [<Ld $register_upper Nn>]  { counter: 1, value: 0 };

                    let result = instruction.act(&mut registers, &mut alu, memory.clone(),&mut false);

                    assert_eq!(false, result);

                    assert_eq!(0x1200, instruction.value);
                }

                #[test]
                fn update_sp_on_tick_0_and_get_next_instruction() {
                    let mut registers = Registers::new(0, 0, 0, 0, 0xC000, 0);
                    let mut alu = ALU::new(registers.f.clone());
                    let memory = Arc::new(Mutex::new(MemoryController::new()));

                    let mut instruction = [<Ld $register_upper Nn>]  { counter: 0, value: 0x1234 };

                    let result = instruction.act(&mut registers, &mut alu, memory.clone(),&mut false);

                    assert_eq!(true, result);

                    assert_eq!(0x1234, registers.$register.get_value());
                }
            }
        }
    }
}

ld_rr_nn!(0x01, bc, Bc);
ld_rr_nn!(0x11, de, De);
ld_rr_nn!(0x21, hl, Hl);

#[macro_export] macro_rules! ld_rr_nn_decode_instruction {
    ($opcode:expr) => {
        return_if_is_instruction!(LdBcNn, $opcode);
        return_if_is_instruction!(LdDeNn, $opcode);
        return_if_is_instruction!(LdHlNn, $opcode);
    }
}
