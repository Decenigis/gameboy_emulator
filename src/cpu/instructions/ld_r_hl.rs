use paste::paste;
use crate::cpu::alu::ALU;
use crate::cpu::instructions::Instruction;
use crate::cpu::register::Register;
use crate::cpu::registers::Registers;
use crate::memory::{MemoryController, MemoryTrait};
use parking_lot::Mutex;
use std::sync::Arc;


macro_rules! ld_r_hl {
    ($opcode:literal, $register:ident, $register_upper:ident) => {
        paste!{
            pub struct [<Ld $register_upper Hl>] {
                counter: u8,
                address: u16
            }

            impl Instruction for [<Ld $register_upper Hl>] {

                #[inline]
                fn from_opcode(opcode: &u8) -> Option<Box<dyn Instruction>> {
                    if *opcode == $opcode {
                        return Some(Box::new([<Ld $register_upper Hl>] { counter: 1, address: 0x0000 }))
                    }
                    None
                }

                fn get_opcode(&self) -> u8 {
                    $opcode
                }

                fn act(&mut self, registers: &mut Registers, _alu: &mut ALU, memory_controller: Arc<Mutex<MemoryController>>, _enable_interrupts: &mut bool, _is_halted: &mut bool) -> bool {
                    if self.counter == 1 {
                        self.address = registers.hl.get_value();
                    }
                    else if self.counter == 0 {
                        registers.$register.borrow_mut().set_value(memory_controller.lock().get(self.address));

                        return true;
                    }

                    self.counter -= 1;
                    false
                }
            }



            #[cfg(test)]
            mod [<ld_ $register _hl_tests>] {
                use super::*;

                reusable_testing_macro!($opcode, [<Ld $register_upper Hl>]);

                #[test]
                fn load_n_on_tick_1() {
                    let mut registers = Registers::new(0, 0, 0, 0xC000, 0xC000, 0);
                    let mut alu = ALU::new(registers.f.clone());
                    let memory = Arc::new(Mutex::new(MemoryController::new()));

                    let mut instruction = [<Ld $register_upper Hl>] { counter: 1, address: 0x0000};

                    let result = instruction.act(&mut registers, &mut alu, memory.clone(),&mut false, &mut false);

                    assert_eq!(false, result);
                    assert_eq!(0xC000, instruction.address);
                }

                #[test]
                fn get_next_instruction_on_tick_0() {
                    let mut registers = Registers::new(0, 0, 0, 0xC000, 0xC000, 0);
                    let mut alu = ALU::new(registers.f.clone());
                    let memory = Arc::new(Mutex::new(MemoryController::new()));

                    memory.lock().set(0xC000, 0x12);

                    let mut instruction = [<Ld $register_upper Hl>] { counter: 0, address: 0xC000 };

                    let result = instruction.act(&mut registers, &mut alu, memory.clone(),&mut false, &mut false);

                    assert_eq!(true, result);
                    assert_eq!(0x12, registers.$register.borrow().get_value());
                }
            }
        }
    }
}

ld_r_hl!(0x46, b, B);
ld_r_hl!(0x4E, c, C);
ld_r_hl!(0x56, d, D);
ld_r_hl!(0x5E, e, E);
ld_r_hl!(0x66, h, H);
ld_r_hl!(0x6E, l, L);



#[macro_export] macro_rules! ld_r_hl_decode_instruction {
    ($opcode:expr) => {
        return_if_is_instruction!(LdBHl, $opcode);   //0x46
        return_if_is_instruction!(LdCHl, $opcode);   //0x4E
        return_if_is_instruction!(LdDHl, $opcode);   //0x56
        return_if_is_instruction!(LdEHl, $opcode);   //0x5E
        return_if_is_instruction!(LdHHl, $opcode);   //0x66
        return_if_is_instruction!(LdLHl, $opcode);   //0x6E
    }
}
