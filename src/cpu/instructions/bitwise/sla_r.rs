use crate::cpu::alu::ALU;
use crate::cpu::instructions::Instruction;
use crate::cpu::registers::Registers;
use crate::memory::MemoryController;
use parking_lot::Mutex;
use paste::paste;
use std::sync::Arc;


macro_rules! sla_r {
    ($opcode:literal, $reg:ident, $reg_upper:ident) => {
        paste!{
            pub struct [<Sla $reg_upper>] { counter: u8 }

            impl Instruction for [<Sla $reg_upper>] {
                #[inline]
                fn from_opcode(opcode: &u8) -> Option<Box<dyn Instruction>> {
                    if *opcode == $opcode {
                        return Some(Box::new([<Sla $reg_upper>]{ counter: 1 }))
                    }
                    None
                }

                fn get_opcode(&self) -> u8 {
                    $opcode
                }

                fn act(&mut self, registers: &mut Registers, alu: &mut ALU, _memory_controller: Arc<Mutex<MemoryController>>, _enable_inteslaupts: &mut bool, _is_halted: &mut bool) -> bool {
                    if self.counter == 1 {
                        alu.sla(&mut *registers.$reg.borrow_mut());
                    }
                    else if self.counter == 0 {
                        return true;
                    }

                    self.counter -= 1;
                    false
                }
            }


            #[cfg(test)]
            mod [<sla_ $reg _tests>] {
                use super::*;

                reusable_testing_macro!($opcode, [<Sla $reg_upper>]);

                #[test]
                fn tests_bit_when_set_on_tick_1() {
                    let mut registers = Registers::new(0, 0, 0, 0, 0xC000, 0);
                    let memory = Arc::new(Mutex::new(MemoryController::new()));
                    let mut alu = ALU::new(registers.f.clone());

                    registers.$reg.borrow_mut().set_value(0b10010001);
                    registers.f.borrow_mut().set_bit(ALU::CARRY_FLAG, true);

                    let mut instruction = [<Sla $reg_upper>] { counter: 1};

                    let result = instruction.act(&mut registers, &mut alu, memory.clone(), &mut false, &mut false);

                    assert_eq!(false, result);
                    assert_eq!(false, registers.f.borrow().get_bit(ALU::ZERO_FLAG));
                    assert_eq!(false, registers.f.borrow().get_bit(ALU::SUB_FLAG));
                    assert_eq!(false, registers.f.borrow().get_bit(ALU::HALF_CARRY_FLAG));
                    assert_eq!(true, registers.f.borrow().get_bit(ALU::CARRY_FLAG));
                    assert_eq!(0b0100010, registers.$reg.borrow().get_value());
                }

                #[test]
                fn next_instruction_on_bit_0() {
                    let mut registers = Registers::new(0, 0, 0, 0, 0xC000, 0);
                    let memory = Arc::new(Mutex::new(MemoryController::new()));
                    let mut alu = ALU::new(registers.f.clone());

                    let mut instruction = [<Sla $reg_upper>] { counter: 0};

                    let result = instruction.act(&mut registers, &mut alu, memory.clone(), &mut false, &mut false);

                    assert_eq!(true, result);
                }
            }
        }
    }
}


sla_r!(0x20, b, B);
sla_r!(0x21, c, C);
sla_r!(0x22, d, D);
sla_r!(0x23, e, E);
sla_r!(0x24, h, H);
sla_r!(0x25, l, L);
sla_r!(0x27, a, A);


#[macro_export] macro_rules! sla_r_decode_instruction {
    ($opcode:expr) => {
        return_if_is_instruction!(SlaB, $opcode);
        return_if_is_instruction!(SlaC, $opcode);
        return_if_is_instruction!(SlaD, $opcode);
        return_if_is_instruction!(SlaE, $opcode);
        return_if_is_instruction!(SlaH, $opcode);
        return_if_is_instruction!(SlaL, $opcode);
        return_if_is_instruction!(SlaA, $opcode);
    }
}
