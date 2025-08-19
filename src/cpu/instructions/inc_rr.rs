use crate::cpu::alu::ALU;
use crate::cpu::instructions::Instruction;
use crate::cpu::register::Register;
use crate::cpu::register16::Register16;
use crate::cpu::registers::Registers;
use crate::memory::MemoryController;
use parking_lot::Mutex;
use paste::paste;
use std::sync::Arc;

macro_rules! inc_rr {
    ($opcode:literal, $register:ident, $register_upper:ident) => {
        paste!{
            pub struct [<Inc $register_upper>] {
                counter: u8
            }

            impl Instruction for [<Inc $register_upper>] {
                #[inline]
                fn from_opcode(opcode: &u8) -> Option<Box<dyn Instruction>> {
                    if *opcode == $opcode {
                        return Some(Box::new([<Inc $register_upper>] { counter: 1}))
                    }
                    None
                }

                fn get_opcode(&self) -> u8 {
                    $opcode
                }

                fn act(&mut self, registers: &mut Registers, alu: &mut ALU, _memory_controller: Arc<Mutex<MemoryController>>, _enable_interrupts: &mut bool, _is_halted: &mut bool) -> bool {
                    if self.counter == 1 {
                        registers.$register.increment();
                    }
                    else if self.counter == 0 {
                        return true;
                    }

                    self.counter -= 1;
                    false
                }
            }

            #[cfg(test)]
            mod [<inc_ $register _tests>] {
                use super::*;

                reusable_testing_macro!($opcode, [<Inc $register_upper>]);

                #[test]
                fn increment_r_on_tick_1() {
                    let mut registers = Registers::new(0, 0, 0, 0, 0xC000, 0);
                    registers.$register.set_value(0x1234);

                    let memory = Arc::new(Mutex::new(MemoryController::new()));
                    let mut alu = ALU::new(registers.f.clone());

                    let mut instruction = [<Inc $register_upper>] { counter: 1};

                    let result = instruction.act(&mut registers, &mut alu, memory.clone(), &mut false, &mut false);

                    assert_eq!(false, result);
                    assert_eq!(0x1235, registers.$register.get_value());
                }

                #[test]
                fn next_instruction_on_tick_0() {
                    let mut registers = Registers::new(0, 0, 0, 0, 0xC000, 0);

                    registers.$register.set_value(0x1235);

                    let memory = Arc::new(Mutex::new(MemoryController::new()));
                    let mut alu = ALU::new(registers.f.clone());

                    let mut instruction = [<Inc $register_upper>] { counter: 0 };

                    let result = instruction.act(&mut registers, &mut alu, memory.clone(), &mut false, &mut false);

                    assert_eq!(true, result);
                    assert_eq!(0x1235, registers.$register.get_value());
                }
            }
        }
    }
}

inc_rr!(0x03, bc, Bc);
inc_rr!(0x13, de, De);
inc_rr!(0x23, hl, Hl);
inc_rr!(0x33, sp, Sp);


#[macro_export] macro_rules! inc_rr_decode_instruction {
    //this really sucks
    ($opcode:expr) => {
        return_if_is_instruction!(IncBc, $opcode);
        return_if_is_instruction!(IncDe, $opcode);
        return_if_is_instruction!(IncHl, $opcode);
        return_if_is_instruction!(IncSp, $opcode);
    }
}
