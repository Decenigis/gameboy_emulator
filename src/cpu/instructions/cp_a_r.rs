use crate::cpu::register8::Register8;
use crate::cpu::alu::ALU;
use crate::cpu::instructions::Instruction;
use crate::cpu::registers::Registers;
use crate::cpu::register::Register;
use crate::memory::MemoryController;
use parking_lot::Mutex;
use paste::paste;
use std::sync::Arc;


macro_rules! cp_a_r {
    ($opcode:literal, $register:ident, $register_upper:ident) => {
        paste! {
            pub struct [<CpA $register_upper>] {}


            impl Instruction for [<CpA $register_upper>] {

                #[inline]
                fn from_opcode(opcode: &u8) -> Option<Box<dyn Instruction>> {
                    if *opcode == $opcode {
                        return Some(Box::new([<CpA $register_upper>] {}))
                    }
                    None
                }


                fn get_opcode(&self) -> u8 {
                    $opcode
                }

                fn act(&mut self, registers: &mut Registers, alu: &mut ALU, _memory_controller: Arc<Mutex<MemoryController>>, _enable_interrupts: &mut bool, _is_halted: &mut bool) -> bool {
                    alu.sub(&mut Register8::new(registers.a.borrow().get_value()), &registers.$register.borrow());

                    true
                }
            }


            #[cfg(test)]
            mod [<cp_a_ $register>] {
                use super::*;
                use crate::cpu::register::Register;
                use crate::memory::MemoryTrait;

                reusable_testing_macro!($opcode, [<CpA $register_upper>]);


                #[test]
                fn cp_value_equality() {
                    let mut registers = Registers::new(0x1200, 0, 0, 0, 0xC001, 0);
                    let memory = Arc::new(Mutex::new(MemoryController::new()));
                    let mut alu = ALU::new(registers.f.clone());

                    registers.$register.borrow_mut().set_value(0x12);

                    let mut instruction = [<CpA $register_upper>] { };

                    let result = instruction.act(&mut registers, &mut alu, memory.clone(), &mut false, &mut false);

                    assert_eq!(true, result);
                    assert_eq!(true, registers.f.borrow().get_bit(ALU::ZERO_FLAG));
                    assert_eq!(false, registers.f.borrow().get_bit(ALU::CARRY_FLAG));
                }

                #[test]
                fn cp_value_greater() {
                    let mut registers = Registers::new(0x1100, 0, 0, 0, 0xC001, 0);
                    let memory = Arc::new(Mutex::new(MemoryController::new()));
                    let mut alu = ALU::new(registers.f.clone());

                    registers.$register.borrow_mut().set_value(0x12);

                    let mut instruction = [<CpA $register_upper>] { };

                    let result = instruction.act(&mut registers, &mut alu, memory.clone(), &mut false, &mut false);

                    assert_eq!(true, result);
                    assert_eq!(false, registers.f.borrow().get_bit(ALU::ZERO_FLAG));
                    assert_eq!(true, registers.f.borrow().get_bit(ALU::CARRY_FLAG));
                }

                #[test]
                fn cp_value_no_equality() {
                    let mut registers = Registers::new(0x1300, 0, 0, 0, 0xC001, 0);
                    let memory = Arc::new(Mutex::new(MemoryController::new()));
                    let mut alu = ALU::new(registers.f.clone());

                    registers.$register.borrow_mut().set_value(0x12);

                    let mut instruction = [<CpA $register_upper>] {  };

                    let result = instruction.act(&mut registers, &mut alu, memory.clone(), &mut false, &mut false);

                    assert_eq!(true, result);
                    assert_eq!(false, registers.f.borrow().get_bit(ALU::ZERO_FLAG));
                    assert_eq!(false, registers.f.borrow().get_bit(ALU::CARRY_FLAG));
                }
            }
        }
    }
}


cp_a_r!(0xB8, b, B);
cp_a_r!(0xB9, c, C);
cp_a_r!(0xBA, d, D);
cp_a_r!(0xBB, e, E);
cp_a_r!(0xBC, h, H);
cp_a_r!(0xBD, l, L);


#[macro_export] macro_rules! cp_a_r_decode_instruction {
    ($opcode:expr) => {
        return_if_is_instruction!(CpAB, $opcode);   //0xB8
        return_if_is_instruction!(CpAC, $opcode);   //0xB9
        return_if_is_instruction!(CpAD, $opcode);   //0xBA
        return_if_is_instruction!(CpAE, $opcode);   //0xBB
        return_if_is_instruction!(CpAH, $opcode);   //0xBC
        return_if_is_instruction!(CpAL, $opcode);   //0xBD
    }
}
