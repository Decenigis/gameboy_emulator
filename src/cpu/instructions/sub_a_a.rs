use crate::cpu::alu::ALU;
use crate::cpu::instructions::Instruction;
use crate::cpu::register::Register;
use crate::cpu::register8::Register8;
use crate::cpu::registers::Registers;
use crate::memory::MemoryController;
use parking_lot::Mutex;
use std::sync::Arc;

pub struct SubAA {}


impl Instruction for SubAA {

    #[inline]
    fn from_opcode(opcode: &u8) -> Option<Box<dyn Instruction>> {
        if *opcode == 0x97 {
            return Some(Box::new(SubAA {}))
        }
        None
    }


    fn get_opcode(&self) -> u8 {
        0x97
    }

    fn act(&mut self, registers: &mut Registers, alu: &mut ALU, _memory_controller: Arc<Mutex<MemoryController>>, _enable_interrupts: &mut bool, _is_halted: &mut bool) -> bool {
        let fake_register = Register8::new(registers.a.borrow().get_value());
        alu.sub(&mut *registers.a.clone().borrow_mut(), &fake_register);

        true
    }
}


#[cfg(test)]
mod sub_a_a {
    use super::*;

    reusable_testing_macro!(0x97, SubAA);
    
    #[test]
    fn sub_a_to_a_on_tick() {
        let mut registers = Registers::new(0, 0, 0, 0, 0xC000, 0);
        registers.a.borrow_mut().set_value(0x12);
        
        let memory = Arc::new(Mutex::new(MemoryController::new()));
        let mut alu = ALU::new(registers.f.clone());
        
        let mut instruction = SubAA {};
        
        let result = instruction.act(&mut registers, &mut alu, memory.clone(), &mut false, &mut false);
        
        assert_eq!(true, result);
        assert_eq!(0x00, registers.a.borrow().get_value());
    }
}
