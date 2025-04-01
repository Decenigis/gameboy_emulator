use std::sync::Arc;
use parking_lot::Mutex;
use crate::cpu::alu::ALU;
use crate::cpu::instructions::Instruction;
use crate::cpu::register::Register;
use crate::cpu::registers::Registers;
use crate::memory::MemoryController;

//Timings for this one might be a bit off due to te 4-cycle alignment

pub struct IncHl {
    counter: u8,
}

impl Instruction for IncHl {

    fn from_opcode(opcode: &u8) -> Option<Box<dyn Instruction>> {
        if *opcode == 0x23 {
            return Some(Box::new(IncHl { counter: 1 }))
        }
        None
    }

    fn get_opcode(&self) -> u8 {
        0x23
    }

    fn act(&mut self, registers: &mut Registers, _alu: &mut ALU, _memory_controller: Arc<Mutex<MemoryController>>, _enable_interrupts: &mut bool) -> bool {
        if self.counter == 1 {
            registers.hl.increment();
        }
        else if self.counter == 0 {
            return true;
        }

        self.counter -= 1;
        false
    }
}



#[cfg(test)]
mod tests {
    use crate::cpu::register::Register;
    use crate::memory::MemoryTrait;
    use super::*;

    #[test]
    fn from_opcode_returns_given_0x23() {
        let instruction = IncHl::from_opcode(&0x23);

        assert_eq!(true, instruction.is_some());
    }

    #[test]
    fn from_opcode_returns_none_given_non_0x23() {
        let instruction = IncHl::from_opcode(&0x00);

        assert_eq!(true, instruction.is_none());
    }

    #[test]
    fn get_opcode_returns_0x23() {
        let instruction = IncHl { counter: 0 };

        assert_eq!(0x23, instruction.get_opcode());
    }

    #[test]
    fn increment_hl_on_tick_1() {
        let mut registers = Registers::new(0, 0, 0, 0x1234, 0xC000, 0);
        let mut alu = ALU::new(registers.f.clone());
        let memory = Arc::new(Mutex::new(MemoryController::new()));

        let mut instruction = IncHl { counter: 1 };

        let result = instruction.act(&mut registers, &mut alu, memory.clone(),&mut false);

        assert_eq!(false, result);
        assert_eq!(0x1235, registers.hl.get_value());
    }

    #[test]
    fn get_next_instruction_on_tick_0() {
        let mut registers = Registers::new(0, 0, 0, 0, 0xC000, 0);
        let mut alu = ALU::new(registers.f.clone());
        let memory = Arc::new(Mutex::new(MemoryController::new()));

        let mut instruction = IncHl { counter: 0 };

        let result = instruction.act(&mut registers, &mut alu, memory.clone(),&mut false);

        assert_eq!(true, result);
    }
}
