use crate::cpu::alu::ALU;
use crate::cpu::instructions::Instruction;
use crate::cpu::register::Register;
use crate::cpu::registers::Registers;
use crate::memory::MemoryController;
use parking_lot::Mutex;
use paste::paste;
use std::sync::Arc;


macro_rules! rrc_r {
    ($opcode:literal, $reg:ident, $reg_upper:ident) => {
        paste!{
            pub struct [<Rrc $reg_upper>] { counter: u8 }

            impl Instruction for [<Rrc $reg_upper>] {
                #[inline]
                fn from_opcode(opcode: &u8) -> Option<Box<dyn Instruction>> {
                    if *opcode == $opcode {
                        return Some(Box::new([<Rrc $reg_upper>]{ counter: 1 }))
                    }
                    None
                }

                fn get_opcode(&self) -> u8 {
                    $opcode
                }

                fn act(&mut self, registers: &mut Registers, alu: &mut ALU, _memory_controller: Arc<Mutex<MemoryController>>, _enable_interrupts: &mut bool, _is_halted: &mut bool) -> bool {
                    if self.counter == 1 {
                        alu.rrc(&mut *registers.$reg.borrow_mut());
                    }
                    else if self.counter == 0 {
                        return true;
                    }

                    self.counter -= 1;
                    false
                }
            }


            #[cfg(test)]
            mod [<rrc_ $reg _tests>] {
                use super::*;

                reusable_testing_macro!($opcode, [<Rrc $reg_upper>]);

                #[test]
                fn rolls_on_tick_1() {
                    let mut registers = Registers::new(0, 0, 0, 0, 0xC000, 0);
                    let memory = Arc::new(Mutex::new(MemoryController::new()));
                    let mut alu = ALU::new(registers.f.clone());

                    registers.$reg.borrow_mut().set_value(0b00100011);

                    let mut instruction = [<Rrc $reg_upper>] { counter: 1};

                    let result = instruction.act(&mut registers, &mut alu, memory.clone(), &mut false, &mut false);

                    assert_eq!(false, result);
                    assert_eq!(false, registers.f.borrow().get_bit(ALU::ZERO_FLAG));
                    assert_eq!(false, registers.f.borrow().get_bit(ALU::SUB_FLAG));
                    assert_eq!(false, registers.f.borrow().get_bit(ALU::HALF_CARRY_FLAG));
                    assert_eq!(true, registers.f.borrow().get_bit(ALU::CARRY_FLAG));
                    assert_eq!(0b10010001, registers.$reg.borrow().get_value());
                }

                #[test]
                fn next_instruction_on_bit_0() {
                    let mut registers = Registers::new(0, 0, 0, 0, 0xC000, 0);
                    let memory = Arc::new(Mutex::new(MemoryController::new()));
                    let mut alu = ALU::new(registers.f.clone());

                    let mut instruction = [<Rrc $reg_upper>] { counter: 0};

                    let result = instruction.act(&mut registers, &mut alu, memory.clone(), &mut false, &mut false);

                    assert_eq!(true, result);
                }
            }
        }
    }
}


rrc_r!(0x08, b, B);
rrc_r!(0x09, c, C);
rrc_r!(0x0A, d, D);
rrc_r!(0x0B, e, E);
rrc_r!(0x0C, h, H);
rrc_r!(0x0D, l, L);
rrc_r!(0x0F, a, A);


#[macro_export] macro_rules! rrc_r_decode_instruction {
    ($opcode:expr) => {
        return_if_is_instruction!(RrcB, $opcode);
        return_if_is_instruction!(RrcC, $opcode);
        return_if_is_instruction!(RrcD, $opcode);
        return_if_is_instruction!(RrcE, $opcode);
        return_if_is_instruction!(RrcH, $opcode);
        return_if_is_instruction!(RrcL, $opcode);
        return_if_is_instruction!(RrcA, $opcode);
    }
}
