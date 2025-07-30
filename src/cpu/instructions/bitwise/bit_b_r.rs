use crate::cpu::alu::ALU;
use crate::cpu::instructions::Instruction;
use crate::cpu::register::Register;
use crate::cpu::registers::Registers;
use crate::memory::MemoryController;
use parking_lot::Mutex;
use paste::paste;
use std::sync::Arc;


macro_rules! bit_b_r {
    ($opcode:literal, $bit:literal, $reg:ident, $reg_upper:ident) => {
        paste!{
            pub struct [<Bit $bit $reg_upper>] { counter: u8  }

            impl Instruction for [<Bit $bit $reg_upper>] {
                #[inline]
                fn from_opcode(opcode: &u8) -> Option<Box<dyn Instruction>> {
                    if *opcode == $opcode {
                        return Some(Box::new([<Bit $bit $reg_upper>]{ counter: 1 }))
                    }
                    None
                }

                fn get_opcode(&self) -> u8 {
                    $opcode
                }

                fn act(&mut self, registers: &mut Registers, _alu: &mut ALU, _memory_controller: Arc<Mutex<MemoryController>>, _enable_interrupts: &mut bool, _is_halted: &mut bool) -> bool {
                    if self.counter == 1 {
                        let mut flags = registers.f.borrow_mut();

                        flags.set_bit(ALU::SUB_FLAG, false);
                        flags.set_bit(ALU::HALF_CARRY_FLAG, true);
                        flags.set_bit(ALU::ZERO_FLAG, !registers.$reg.borrow().get_bit($bit));
                    }
                    else if self.counter == 0 {
                        return true;
                    }

                    self.counter -= 1;
                    false
                }
            }


            #[cfg(test)]
            mod [<bit_ $bit _ $reg _tests>] {
                use super::*;

                reusable_testing_macro!($opcode, [<Bit $bit $reg_upper>]);

                #[test]
                fn tests_bit_when_set_on_tick_1() {
                    let mut registers = Registers::new(0, 0, 0, 0, 0xC000, 0);
                    let memory = Arc::new(Mutex::new(MemoryController::new()));
                    let mut alu = ALU::new(registers.f.clone());

                    registers.$reg.borrow_mut().set_bit($bit, true);

                    let mut instruction = [<Bit $bit $reg_upper>] { counter: 1};

                    let result = instruction.act(&mut registers, &mut alu, memory.clone(), &mut false, &mut false);

                    assert_eq!(false, result);
                    assert_eq!(false, registers.f.borrow().get_bit(ALU::SUB_FLAG));
                    assert_eq!(true, registers.f.borrow().get_bit(ALU::HALF_CARRY_FLAG));
                    assert_eq!(false, registers.f.borrow().get_bit(ALU::ZERO_FLAG));
                }

                #[test]
                fn tests_bit_when_unset_on_tick_1() {
                    let mut registers = Registers::new(0, 0, 0, 0, 0xC000, 0);
                    let memory = Arc::new(Mutex::new(MemoryController::new()));
                    let mut alu = ALU::new(registers.f.clone());

                    registers.$reg.borrow_mut().set_bit($bit, false);

                    let mut instruction = [<Bit $bit $reg_upper>] { counter: 1};

                    let result = instruction.act(&mut registers, &mut alu, memory.clone(), &mut false, &mut false);

                    assert_eq!(false, result);
                    assert_eq!(false, registers.f.borrow().get_bit(ALU::SUB_FLAG));
                    assert_eq!(true, registers.f.borrow().get_bit(ALU::HALF_CARRY_FLAG));
                    assert_eq!(true, registers.f.borrow().get_bit(ALU::ZERO_FLAG));
                }


                #[test]
                fn returns_on_tick_0() {
                    let mut registers = Registers::new(0, 0, 0, 0, 0xC000, 0);
                    let memory = Arc::new(Mutex::new(MemoryController::new()));
                    let mut alu = ALU::new(registers.f.clone());

                    let mut instruction = [<Bit $bit $reg_upper>] { counter: 0 };

                    let result = instruction.act(&mut registers, &mut alu, memory.clone(), &mut false, &mut false);

                    assert_eq!(true, result);
                }
            }
        }
    }
}

bit_b_r!(0x40, 0, b, B);
bit_b_r!(0x41, 0, c, C);
bit_b_r!(0x42, 0, d, D);
bit_b_r!(0x43, 0, e, E);
bit_b_r!(0x44, 0, h, H);
bit_b_r!(0x45, 0, l, L);
bit_b_r!(0x47, 0, a, A);
bit_b_r!(0x48, 1, b, B);
bit_b_r!(0x49, 1, c, C);
bit_b_r!(0x4A, 1, d, D);
bit_b_r!(0x4B, 1, e, E);
bit_b_r!(0x4C, 1, h, H);
bit_b_r!(0x4D, 1, l, L);
bit_b_r!(0x4F, 1, a, A);
bit_b_r!(0x50, 2, b, B);
bit_b_r!(0x51, 2, c, C);
bit_b_r!(0x52, 2, d, D);
bit_b_r!(0x53, 2, e, E);
bit_b_r!(0x54, 2, h, H);
bit_b_r!(0x55, 2, l, L);
bit_b_r!(0x57, 2, a, A);
bit_b_r!(0x58, 3, b, B);
bit_b_r!(0x59, 3, c, C);
bit_b_r!(0x5A, 3, d, D);
bit_b_r!(0x5B, 3, e, E);
bit_b_r!(0x5C, 3, h, H);
bit_b_r!(0x5D, 3, l, L);
bit_b_r!(0x5F, 3, a, A);
bit_b_r!(0x60, 4, b, B);
bit_b_r!(0x61, 4, c, C);
bit_b_r!(0x62, 4, d, D);
bit_b_r!(0x63, 4, e, E);
bit_b_r!(0x64, 4, h, H);
bit_b_r!(0x65, 4, l, L);
bit_b_r!(0x67, 4, a, A);
bit_b_r!(0x68, 5, b, B);
bit_b_r!(0x69, 5, c, C);
bit_b_r!(0x6A, 5, d, D);
bit_b_r!(0x6B, 5, e, E);
bit_b_r!(0x6C, 5, h, H);
bit_b_r!(0x6D, 5, l, L);
bit_b_r!(0x6F, 5, a, A);
bit_b_r!(0x70, 6, b, B);
bit_b_r!(0x71, 6, c, C);
bit_b_r!(0x72, 6, d, D);
bit_b_r!(0x73, 6, e, E);
bit_b_r!(0x74, 6, h, H);
bit_b_r!(0x75, 6, l, L);
bit_b_r!(0x77, 6, a, A);
bit_b_r!(0x78, 7, b, B);
bit_b_r!(0x79, 7, c, C);
bit_b_r!(0x7A, 7, d, D);
bit_b_r!(0x7B, 7, e, E);
bit_b_r!(0x7C, 7, h, H);
bit_b_r!(0x7D, 7, l, L);
bit_b_r!(0x7F, 7, a, A);



#[macro_export] macro_rules! bit_b_r_decode_instruction {
    ($opcode:expr) => {
        return_if_is_instruction!(Bit0B, $opcode);
        return_if_is_instruction!(Bit0C, $opcode);
        return_if_is_instruction!(Bit0D, $opcode);
        return_if_is_instruction!(Bit0E, $opcode);
        return_if_is_instruction!(Bit0H, $opcode);
        return_if_is_instruction!(Bit0L, $opcode);
        return_if_is_instruction!(Bit0A, $opcode);
        return_if_is_instruction!(Bit1B, $opcode);
        return_if_is_instruction!(Bit1C, $opcode);
        return_if_is_instruction!(Bit1D, $opcode);
        return_if_is_instruction!(Bit1E, $opcode);
        return_if_is_instruction!(Bit1H, $opcode);
        return_if_is_instruction!(Bit1L, $opcode);
        return_if_is_instruction!(Bit1A, $opcode);
        return_if_is_instruction!(Bit2B, $opcode);
        return_if_is_instruction!(Bit2C, $opcode);
        return_if_is_instruction!(Bit2D, $opcode);
        return_if_is_instruction!(Bit2E, $opcode);
        return_if_is_instruction!(Bit2H, $opcode);
        return_if_is_instruction!(Bit2L, $opcode);
        return_if_is_instruction!(Bit2A, $opcode);
        return_if_is_instruction!(Bit3B, $opcode);
        return_if_is_instruction!(Bit3C, $opcode);
        return_if_is_instruction!(Bit3D, $opcode);
        return_if_is_instruction!(Bit3E, $opcode);
        return_if_is_instruction!(Bit3H, $opcode);
        return_if_is_instruction!(Bit3L, $opcode);
        return_if_is_instruction!(Bit3A, $opcode);
        return_if_is_instruction!(Bit4B, $opcode);
        return_if_is_instruction!(Bit4C, $opcode);
        return_if_is_instruction!(Bit4D, $opcode);
        return_if_is_instruction!(Bit4E, $opcode);
        return_if_is_instruction!(Bit4H, $opcode);
        return_if_is_instruction!(Bit4L, $opcode);
        return_if_is_instruction!(Bit4A, $opcode);
        return_if_is_instruction!(Bit5B, $opcode);
        return_if_is_instruction!(Bit5C, $opcode);
        return_if_is_instruction!(Bit5D, $opcode);
        return_if_is_instruction!(Bit5E, $opcode);
        return_if_is_instruction!(Bit5H, $opcode);
        return_if_is_instruction!(Bit5L, $opcode);
        return_if_is_instruction!(Bit5A, $opcode);
        return_if_is_instruction!(Bit6B, $opcode);
        return_if_is_instruction!(Bit6C, $opcode);
        return_if_is_instruction!(Bit6D, $opcode);
        return_if_is_instruction!(Bit6E, $opcode);
        return_if_is_instruction!(Bit6H, $opcode);
        return_if_is_instruction!(Bit6L, $opcode);
        return_if_is_instruction!(Bit6A, $opcode);
        return_if_is_instruction!(Bit7B, $opcode);
        return_if_is_instruction!(Bit7C, $opcode);
        return_if_is_instruction!(Bit7D, $opcode);
        return_if_is_instruction!(Bit7E, $opcode);
        return_if_is_instruction!(Bit7H, $opcode);
        return_if_is_instruction!(Bit7L, $opcode);
        return_if_is_instruction!(Bit7A, $opcode);
    }
}
