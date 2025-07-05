use crate::cpu::alu::ALU;
use crate::cpu::instructions::Instruction;
use crate::cpu::register::Register;
use crate::cpu::registers::Registers;
use crate::memory::{MemoryController, MemoryTrait};
use parking_lot::Mutex;
use paste::paste;
use std::sync::Arc;


macro_rules! push_rr {
    ($opcode:literal, $register:ident, $register_upper:ident) => {
        paste! {
            pub struct [<Push $register_upper>] {
                counter: u8
            }

            impl Instruction for [<Push $register_upper>] {

                #[inline]
                fn from_opcode(opcode: &u8) -> Option<Box<dyn Instruction>> {
                    if *opcode == $opcode {
                        return Some(Box::new([<Push $register_upper>] { counter: 3 }))
                    }
                    None
                }

                fn get_opcode(&self) -> u8 {
                    $opcode
                }

                //maybe wrong ordering, shouldn't have too much bearing.
                fn act(&mut self, registers: &mut Registers, _alu: &mut ALU, memory_controller: Arc<Mutex<MemoryController>>, _enable_interrupts: &mut bool, _is_halted: &mut bool) -> bool {
                    if self.counter == 3 {
                        registers.sp.decrement();
                        memory_controller.lock().set(registers.sp.get_value(), ((registers.$register.get_value() & 0xFF00) >> 8) as u8);

                    }
                    else if self.counter == 2 {
                        registers.sp.decrement();
                        memory_controller.lock().set(registers.sp.get_value(), (registers.$register.get_value() & 0xFF) as u8);
                    }
                    else if self.counter == 0 {
                        return true;
                    }

                    self.counter -= 1;
                    false
                }
            }

            #[cfg(test)]
            mod [<push_ $register>] {
                use super::*;

                reusable_testing_macro!($opcode, [<Push $register_upper>]);

                #[test]
                fn push_low_on_tick_3() {
                    let mut registers = Registers::new(0, 0, 0, 0, 0xC000, 0xE000);
                    registers.$register.set_value(0x1234);

                    let mut alu = ALU::new(registers.f.clone());
                    let memory = Arc::new(Mutex::new(MemoryController::new()));

                    let mut instruction = [<Push $register_upper>] { counter: 3 };

                    let result = instruction.act(&mut registers, &mut alu, memory.clone(), &mut false, &mut false);

                    assert_eq!(false, result);
                    assert_eq!(0x12, memory.lock().get(0xDFFF));
                    assert_eq!(0xDFFF, registers.sp.get_value());
                }

                #[test]
                fn saves_address_to_stack_on_tick_2() {
                    let mut registers = Registers::new(0, 0, 0, 0, 0xC000, 0xDFFF);
                    registers.$register.set_value(0x1234);
                    let mut alu = ALU::new(registers.f.clone());
                    let memory = Arc::new(Mutex::new(MemoryController::new()));

                    let mut instruction = [<Push $register_upper>] { counter: 2 };

                    let result = instruction.act(&mut registers, &mut alu, memory.clone(), &mut false, &mut false);

                    assert_eq!(false, result);
                    assert_eq!(0x34, memory.lock().get(0xDFFE));
                    assert_eq!(0xDFFE, registers.sp.get_value());
                }


                #[test]
                fn saves_address_to_stack_on_tick_1() {
                    let mut registers = Registers::new(0, 0, 0, 0, 0xC000, 0xDFFE);
                    let mut alu = ALU::new(registers.f.clone());
                    let memory = Arc::new(Mutex::new(MemoryController::new()));

                    let mut instruction = [<Push $register_upper>] { counter: 1 };

                    let result = instruction.act(&mut registers, &mut alu, memory.clone(), &mut false, &mut false);

                    assert_eq!(false, result);
                    assert_eq!(0xDFFE, registers.sp.get_value());
                }

                #[test]
                fn update_pc_on_tick_0_and_get_next_instruction() {
                    let mut registers = Registers::new(0, 0, 0, 0, 0xC000, 0xDFFE);
                    let mut alu = ALU::new(registers.f.clone());
                    let memory = Arc::new(Mutex::new(MemoryController::new()));

                    let mut instruction = [<Push $register_upper>] { counter: 0 };

                    let result = instruction.act(&mut registers, &mut alu, memory.clone(), &mut false, &mut false);

                    assert_eq!(true, result);
                    assert_eq!(0xDFFE, registers.sp.get_value());
                }
            }
        }
    };
}


push_rr!(0xC5, bc, BC);
push_rr!(0xD5, de, DE);
push_rr!(0xE5, hl, HL);
push_rr!(0xF5, af, AF);


#[macro_export] macro_rules! push_rr_decode_instruction {
    ($opcode:expr) => {
        return_if_is_instruction!(PushBC, $opcode);   //0xC5
        return_if_is_instruction!(PushDE, $opcode);   //0xD5
        return_if_is_instruction!(PushHL, $opcode);   //0xE5
        return_if_is_instruction!(PushAF, $opcode);   //0xF5
    }
}
