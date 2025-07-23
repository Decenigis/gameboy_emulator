use paste::paste;
use crate::cpu::alu::ALU;
use crate::cpu::instructions::Instruction;
use crate::cpu::register::Register;
use crate::cpu::registers::Registers;
use crate::memory::{MemoryController, MemoryTrait};
use parking_lot::Mutex;
use std::sync::Arc;


macro_rules! ld_hl_r {
    ($opcode:literal, $register:ident, $register_upper:ident) => {
        paste!{
            pub struct [<Ld Hl $register_upper>] {
                counter: u8,
                address: u16
            }

            impl Instruction for [<Ld Hl $register_upper>] {

                #[inline]
                fn from_opcode(opcode: &u8) -> Option<Box<dyn Instruction>> {
                    if *opcode == $opcode {
                        return Some(Box::new([<Ld Hl $register_upper>] { counter: 1, address: 0x0000 }))
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
                        memory_controller.lock().set(self.address, registers.$register.borrow().get_value());

                        return true;
                    }

                    self.counter -= 1;
                    false
                }
            }



            #[cfg(test)]
            mod [<ld_hl_ $register _tests>] {
                use super::*;

                reusable_testing_macro!($opcode, [<Ld Hl $register_upper>]);

                #[test]
                fn load_n_on_tick_1() {
                    let mut registers = Registers::new(0, 0, 0, 0xC000, 0xC000, 0);
                    let mut alu = ALU::new(registers.f.clone());
                    let memory = Arc::new(Mutex::new(MemoryController::new()));

                    let mut instruction = [<Ld Hl $register_upper>] { counter: 1, address: 0x0000};

                    let result = instruction.act(&mut registers, &mut alu, memory.clone(),&mut false, &mut false);

                    assert_eq!(false, result);
                    assert_eq!(0xC000, instruction.address);
                }

                #[test]
                fn get_next_instruction_on_tick_0() {
                    let mut registers = Registers::new(0, 0, 0, 0xC000, 0xC000, 0);
                    let mut alu = ALU::new(registers.f.clone());
                    let memory = Arc::new(Mutex::new(MemoryController::new()));

                    registers.$register.borrow_mut().set_value(0x12);

                    let mut instruction = [<Ld Hl $register_upper>] { counter: 0, address: 0xC000 };

                    let result = instruction.act(&mut registers, &mut alu, memory.clone(),&mut false, &mut false);

                    assert_eq!(true, result);
                    assert_eq!(0x12, memory.lock().get(0xC000));
                }
            }
        }
    }
}

ld_hl_r!(0x70, b, B);
ld_hl_r!(0x71, c, C);
ld_hl_r!(0x72, d, D);
ld_hl_r!(0x73, e, E);
ld_hl_r!(0x74, h, H);
ld_hl_r!(0x75, l, L);



#[macro_export] macro_rules! ld_hl_r_decode_instruction {
    ($opcode:expr) => {
        return_if_is_instruction!(LdHlB, $opcode);   //0x70
        return_if_is_instruction!(LdHlC, $opcode);   //0x71
        return_if_is_instruction!(LdHlD, $opcode);   //0x72
        return_if_is_instruction!(LdHlE, $opcode);   //0x73
        return_if_is_instruction!(LdHlH, $opcode);   //0x74
        return_if_is_instruction!(LdHlL, $opcode);   //0x75
    }
}
