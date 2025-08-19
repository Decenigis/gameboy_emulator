use std::sync::Arc;
use parking_lot::Mutex;
use crate::cpu::alu::ALU;
use crate::cpu::instructions::cp_a_a::CpAA;
use crate::cpu::instructions::Instruction;
use crate::cpu::register8::Register8;
use crate::cpu::register::Register;
use crate::cpu::registers::Registers;
use crate::memory::MemoryController;

pub struct DAA {
}

impl Instruction for DAA {

    #[inline]
    fn from_opcode(opcode: &u8) -> Option<Box<dyn Instruction>> {
        if *opcode == 0x27 {
            return Some(Box::new(DAA {}))
        }
        None
    }

    fn get_opcode(&self) -> u8 {
        0x27
    }

    fn act(&mut self, registers: &mut Registers, _alu: &mut ALU, _memory_controller: Arc<Mutex<MemoryController>>, _enable_interrupts: &mut bool, _is_halted: &mut bool) -> bool {
        let mut offset = 0;


        let a_value = registers.a.borrow().get_value();
        let half_carry = registers.f.borrow().get_bit(ALU::HALF_CARRY_FLAG);
        let carry = registers.f.borrow().get_bit(ALU::CARRY_FLAG);
        let subtract = registers.f.borrow().get_bit(ALU::SUB_FLAG);

        if ((!subtract) && ((a_value & 0xF) > 0x09)) || half_carry {
            offset |= 0x06;
        }

        if ((!subtract) && (a_value > 0x99)) || carry {
            offset |= 0x60;
            registers.f.borrow_mut().set_bit(ALU::CARRY_FLAG, true);
        }

        if !subtract {
            registers.a.borrow_mut().wrapping_add(&Register8::new(offset));
        } else {
            registers.a.borrow_mut().wrapping_sub(&Register8::new(offset));
        }

        registers.f.borrow_mut().set_bit(ALU::HALF_CARRY_FLAG, false);
        registers.f.borrow_mut().set_bit(ALU::ZERO_FLAG, registers.a.borrow().get_value() == 0);

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpu::register::Register;

    reusable_testing_macro!(0x27, DAA);

    #[test]
    fn act_immediately_returns_true() {
        let mut registers = Registers::new(0, 0, 0, 0, 0, 0);
        let mut alu = ALU::new(registers.f.clone());

        let mut instruction = DAA {};

        let result = instruction.act(&mut registers, &mut alu, Arc::new(Mutex::new(MemoryController::new())), &mut false, &mut false);

        assert_eq!(true, result);
    }

    #[test]
    fn act_results_in_correct_flags() {
        let mut registers = Registers::new(0x00, 0, 0, 0, 0, 0);
        let mut alu = ALU::new(registers.f.clone());

        let mut instruction = DAA {};
        
        instruction.act(&mut registers, &mut alu, Arc::new(Mutex::new(MemoryController::new())), &mut false, &mut false);
        
        assert_eq!(false, registers.f.borrow().get_bit(ALU::SUB_FLAG));
        assert_eq!(false, registers.f.borrow().get_bit(ALU::HALF_CARRY_FLAG));

    }
    
    //ngl testing daa is really hard so leaving it like this for now
}