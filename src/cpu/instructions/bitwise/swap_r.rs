use crate::cpu::alu::ALU;
use crate::cpu::instructions::Instruction;
use crate::cpu::register::Register;
use crate::cpu::registers::Registers;
use crate::memory::MemoryController;
use parking_lot::Mutex;
use paste::paste;
use std::sync::Arc;


macro_rules! swap_r {
    ($opcode:literal, $reg:ident, $reg_upper:ident) => {
        paste!{
            pub struct [<Swap $reg_upper>] {  }

            impl Instruction for [<Swap $reg_upper>] {
                #[inline]
                fn from_opcode(opcode: &u8) -> Option<Box<dyn Instruction>> {
                    if *opcode == $opcode {
                        return Some(Box::new([<Swap $reg_upper>]{ }))
                    }
                    None
                }

                fn get_opcode(&self) -> u8 {
                    $opcode
                }

                fn act(&mut self, registers: &mut Registers, _alu: &mut ALU, _memory_controller: Arc<Mutex<MemoryController>>, _enable_interrupts: &mut bool, _is_halted: &mut bool) -> bool {
                    let mut flags = registers.f.borrow_mut();

                    flags.set_bit(ALU::SUB_FLAG, false);
                    flags.set_bit(ALU::HALF_CARRY_FLAG, false);
                    flags.set_bit(ALU::CARRY_FLAG, false);

                    let old_value = registers.$reg.borrow().get_value();
                    registers.$reg.borrow_mut().set_value(old_value << 4 | old_value >> 4);

                    flags.set_bit(ALU::ZERO_FLAG, registers.$reg.borrow().is_zero());

                    true
                }
            }


            #[cfg(test)]
            mod [<swap_ $reg _tests>] {
                use super::*;

                reusable_testing_macro!($opcode, [<Swap $reg_upper>]);

                #[test]
                fn swaps_when_nonzero() {
                    let mut registers = Registers::new(0, 0, 0, 0, 0xC000, 0);
                    let memory = Arc::new(Mutex::new(MemoryController::new()));
                    let mut alu = ALU::new(registers.f.clone());

                    registers.$reg.borrow_mut().set_value(0x12);

                    let mut instruction = [<Swap $reg_upper>] {};

                    let result = instruction.act(&mut registers, &mut alu, memory.clone(), &mut false, &mut false);

                    assert_eq!(true, result);
                    assert_eq!(false, registers.f.borrow().get_bit(ALU::SUB_FLAG));
                    assert_eq!(false, registers.f.borrow().get_bit(ALU::HALF_CARRY_FLAG));
                    assert_eq!(false, registers.f.borrow().get_bit(ALU::CARRY_FLAG));
                    assert_eq!(false, registers.f.borrow().get_bit(ALU::ZERO_FLAG));
                    assert_eq!(0x21, registers.$reg.borrow().get_value());
                }

                #[test]
                fn swaps_when_zero() {
                    let mut registers = Registers::new(0, 0, 0, 0, 0xC000, 0);
                    let memory = Arc::new(Mutex::new(MemoryController::new()));
                    let mut alu = ALU::new(registers.f.clone());

                    registers.$reg.borrow_mut().set_value(0x00);

                    let mut instruction = [<Swap $reg_upper>] {};

                    let result = instruction.act(&mut registers, &mut alu, memory.clone(), &mut false, &mut false);

                    assert_eq!(true, result);
                    assert_eq!(false, registers.f.borrow().get_bit(ALU::SUB_FLAG));
                    assert_eq!(false, registers.f.borrow().get_bit(ALU::HALF_CARRY_FLAG));
                    assert_eq!(false, registers.f.borrow().get_bit(ALU::CARRY_FLAG));
                    assert_eq!(true, registers.f.borrow().get_bit(ALU::ZERO_FLAG));
                    assert_eq!(0x0, registers.$reg.borrow().get_value());
                }
            }
        }
    }
}


swap_r!(0x30, b, B);
swap_r!(0x31, c, C);
swap_r!(0x32, d, D);
swap_r!(0x33, e, E);
swap_r!(0x34, h, H);
swap_r!(0x35, l, L);
swap_r!(0x37, a, A);


#[macro_export] macro_rules! swap_r_decode_instruction {
    ($opcode:expr) => {
        return_if_is_instruction!(SwapB, $opcode);
        return_if_is_instruction!(SwapC, $opcode);
        return_if_is_instruction!(SwapD, $opcode);
        return_if_is_instruction!(SwapE, $opcode);
        return_if_is_instruction!(SwapH, $opcode);
        return_if_is_instruction!(SwapL, $opcode);
        return_if_is_instruction!(SwapA, $opcode);
    }
}
