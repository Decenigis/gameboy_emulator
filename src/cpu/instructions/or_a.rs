use crate::cpu::alu::ALU;
use crate::cpu::instructions::Instruction;
use crate::cpu::registers::Registers;
use crate::memory::MemoryController;
use parking_lot::Mutex;
use std::sync::Arc;

pub struct OrA {}

impl Instruction for OrA {

    #[inline]
    fn from_opcode(opcode: &u8) -> Option<Box<dyn Instruction>> {
        if *opcode == 0xB7 {
            return Some(Box::new(OrA {}))
        }
        None
    }

    fn get_opcode(&self) -> u8 {
        0xB7
    }

    fn act(&mut self, registers: &mut Registers, alu: &mut ALU, _memory_controller: Arc<Mutex<MemoryController>>, _enable_interrupts: &mut bool, _is_halted: &mut bool) -> bool {
        alu.or_internal(registers.a.clone(), registers.a.clone());

        true
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpu::register::Register;

    reusable_testing_macro!(0xB7, OrA);

    #[test]
    fn act_immediately_returns_true() {
        let mut registers = Registers::new(0, 0, 0, 0, 0, 0);
        let mut alu = ALU::new(registers.f.clone());

        let mut instruction = OrA {};

        let result = instruction.act(&mut registers, &mut alu, Arc::new(Mutex::new(MemoryController::new())) ,&mut false, &mut false);

        assert_eq!(true, result);
    }

    #[test]
    fn act_results_in_zero() {
        let mut registers = Registers::new(0xFF00, 0, 0, 0, 0, 0);
        let mut alu = ALU::new(registers.f.clone());

        let mut instruction = OrA {};

        instruction.act(&mut registers, &mut alu, Arc::new(Mutex::new(MemoryController::new())), &mut false, &mut false);

        assert_eq!(0x00, registers.a.borrow().get_value());
    }
}
