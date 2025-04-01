use std::sync::Arc;
use parking_lot::Mutex;
use crate::cpu::alu::ALU;
use crate::cpu::instructions::Instruction;
use crate::cpu::register::Register;
use crate::cpu::registers::Registers;
use crate::memory::MemoryController;

//Timings for this one might be a bit off due to te 4-cycle alignment

pub struct DecBc {
    counter: u8,
}

impl Instruction for DecBc {

    fn from_opcode(opcode: &u8) -> Option<Box<dyn Instruction>> {
        if *opcode == 0x0B {
            return Some(Box::new(DecBc { counter: 1 }))
        }
        None
    }

    fn get_opcode(&self) -> u8 {
        0x0B
    }

    fn act(&mut self, registers: &mut Registers, _alu: &mut ALU, _memory_controller: Arc<Mutex<MemoryController>>, _enable_interrupts: &mut bool) -> bool {
        if self.counter == 1 {
            registers.bc.increment();
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
    fn from_opcode_returns_given_0x0b() {
        let instruction = DecBc::from_opcode(&0x0B);

        assert_eq!(true, instruction.is_some());
    }

    #[test]
    fn from_opcode_returns_none_given_non_0x0b() {
        let instruction = DecBc::from_opcode(&0x00);

        assert_eq!(true, instruction.is_none());
    }

    #[test]
    fn get_opcode_returns_0x0b() {
        let instruction = DecBc { counter: 0 };

        assert_eq!(0x0B, instruction.get_opcode());
    }

    #[test]
    fn increment_hl_on_tick_1() {
        let mut registers = Registers::new(0, 0x1234, 0, 0, 0xC000, 0);
        let mut alu = ALU::new(registers.f.clone());
        let memory = Arc::new(Mutex::new(MemoryController::new()));

        let mut instruction = DecBc { counter: 1 };

        let result = instruction.act(&mut registers, &mut alu, memory.clone(),&mut false);

        assert_eq!(false, result);
        assert_eq!(0x1235, registers.bc.get_value());
    }

    #[test]
    fn get_next_instruction_on_tick_0() {
        let mut registers = Registers::new(0, 0, 0, 0, 0xC000, 0);
        let mut alu = ALU::new(registers.f.clone());
        let memory = Arc::new(Mutex::new(MemoryController::new()));

        let mut instruction = DecBc { counter: 0 };

        let result = instruction.act(&mut registers, &mut alu, memory.clone(),&mut false);

        assert_eq!(true, result);
    }
}
