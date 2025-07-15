use crate::cpu::register::Register;
use crate::cpu::register16::Register16;
use crate::cpu::alu::ALU;
use crate::cpu::instructions::Instruction;
use crate::cpu::registers::Registers;
use crate::memory::MemoryController;
use parking_lot::Mutex;
use paste::paste;
use std::sync::Arc;


macro_rules! add_hl_rr {
    ($opcode:literal, $register:ident, $register_upper:ident) => {
        paste! {
            pub struct [<AddHl $register_upper>] {
                counter: u8
            }


            impl Instruction for [<AddHl $register_upper>] {

                #[inline]
                fn from_opcode(opcode: &u8) -> Option<Box<dyn Instruction>> {
                    if *opcode == $opcode {
                        return Some(Box::new([<AddHl $register_upper>] { counter: 1 }))
                    }
                    None
                }


                fn get_opcode(&self) -> u8 {
                    $opcode
                }

                fn act(&mut self, registers: &mut Registers, alu: &mut ALU, _memory_controller: Arc<Mutex<MemoryController>>, _enable_interrupts: &mut bool, _is_halted: &mut bool) -> bool {
                    if self.counter == 1 {
                        let temp_register = Register16::new(registers.$register.get_value());
                        alu.add(&mut registers.hl, &temp_register);
                    }
                    else if self.counter == 0 {
                        return true;
                    }

                    self.counter -= 1;
                    false
                }
            }


            #[cfg(test)]
            mod [<add_hl_ $register>] {
                use crate::cpu::register::Register;
                use super::*;

                reusable_testing_macro!($opcode, [<AddHl $register_upper>]);

                #[test]
                fn add_rr_to_hl_on_tick_1() {
                    let mut registers = Registers::new(0, 0, 0, 0, 0xC000, 0);
                    registers.hl.set_value(0x1234);
                    registers.$register.set_value(0x1234);

                    let memory = Arc::new(Mutex::new(MemoryController::new()));
                    let mut alu = ALU::new(registers.f.clone());

                    let mut instruction = [<AddHl $register_upper>] { counter: 1};

                    let result = instruction.act(&mut registers, &mut alu, memory.clone(), &mut false, &mut false);

                    assert_eq!(false, result);
                    assert_eq!(0x2468, registers.hl.get_value());
                }

                #[test]
                fn add_r_to_a_on_tick_0() {
                    let mut registers = Registers::new(0, 0, 0, 0, 0xC000, 0);
                    let memory = Arc::new(Mutex::new(MemoryController::new()));
                    let mut alu = ALU::new(registers.f.clone());

                    let mut instruction = [<AddHl $register_upper>] { counter: 0 };

                    let result = instruction.act(&mut registers, &mut alu, memory.clone(), &mut false, &mut false);

                    assert_eq!(true, result);
                }
            }
        }
    }
}


add_hl_rr!(0x09, bc, Bc);
add_hl_rr!(0x19, de, De);
add_hl_rr!(0x29, hl, Hl);
add_hl_rr!(0x39, sp, Sp);


#[macro_export] macro_rules! add_hl_rr_decode_instruction {
    ($opcode:expr) => {
        return_if_is_instruction!(AddHlBc, $opcode);   //0x09
        return_if_is_instruction!(AddHlDe, $opcode);   //0x19
        return_if_is_instruction!(AddHlHl, $opcode);   //0x29
        return_if_is_instruction!(AddHlSp, $opcode);   //0x39
    }
}
