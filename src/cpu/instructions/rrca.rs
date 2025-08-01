use std::sync::Arc;
use parking_lot::Mutex;
use crate::cpu::alu::ALU;
use crate::cpu::instructions::Instruction;
use crate::cpu::registers::Registers;
use crate::memory::MemoryController;

pub struct RrcA {}

impl Instruction for RrcA {
    fn from_opcode(opcode: &u8) -> Option<Box<dyn Instruction>>
    where
        Self: Sized
    {
        if *opcode == 0x0F {
            return Some(Box::new(RrcA {}))
        }
        None
    }

    fn get_opcode(&self) -> u8 {
        0x0F
    }

    fn act(&mut self, registers: &mut Registers, alu: &mut ALU, _memory_controller: Arc<Mutex<MemoryController>>, _enable_interrupts: &mut bool, _is_halted: &mut bool) -> bool {
        alu.rrc(&mut registers.a.borrow_mut());

        true
    }
}


#[cfg(test)]
mod tests {
    use crate::cpu::register::Register;
    use super::*;

    reusable_testing_macro!(0x0F, RrcA);

    #[test]
    fn rolls_and_gets_next_instruction_on_tick_0() {
        let mut registers = Registers::new(0, 0, 0, 0, 0xC000, 0);
        let memory = Arc::new(Mutex::new(MemoryController::new()));
        let mut alu = ALU::new(registers.f.clone());

        registers.a.borrow_mut().set_value(0b00010001);

        let mut instruction = RrcA { };

        let result = instruction.act(&mut registers, &mut alu, memory.clone(), &mut false, &mut false);

        assert_eq!(true, result);
        assert_eq!(0b10001000, registers.a.borrow().get_value());
        assert_eq!(true, registers.f.borrow().get_bit(ALU::CARRY_FLAG));
    }
}
