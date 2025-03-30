use std::sync::Arc;
use parking_lot::Mutex;
use crate::cpu::alu::ALU;
use crate::cpu::CPU;
use crate::cpu::instructions::{decode_instruction, Instruction, Nop};
use crate::cpu::interrupt::Interrupt;
use crate::cpu::register::Register;
use crate::cpu::registers::Registers;
use crate::memory::{MemoryController, MemoryTrait};

pub struct GameBoyCPU {
    registers: Registers,
    alu: ALU,
    enable_interrupts: bool,
    current_instruction: Box<dyn Instruction>
}


impl CPU for GameBoyCPU {

    fn clock (&mut self, memory: Arc<Mutex<MemoryController>>) {
        let instruction_finished = self.current_instruction.act(&mut self.registers, &mut self.alu, memory.clone(), &mut self.enable_interrupts);

        if instruction_finished {
            self.load_next_instruction(memory)
        }
    }

    fn try_interrupt(&mut self, memory: Arc<Mutex<MemoryController>>, interrupt: Interrupt) {
        todo!()
    }
}

impl GameBoyCPU {

    pub fn new_with_nop() -> Self {
        Self::new(Box::new(Nop {}))
    }

    pub fn new(first_instruction: Box<dyn Instruction>) -> Self {
        let registers = Registers::new(
            0,
            0,
            0,
            0,
            0x100,
            0xFFFF
        );
        let f = registers.f.clone();

        Self {
            registers,
            alu: ALU::new(f),
            enable_interrupts: false,
            current_instruction: first_instruction,
        }
    }


    fn load_next_instruction (&mut self, memory: Arc<Mutex<MemoryController>>) {
        let opcode = memory.lock().get(self.registers.pc.get_value());

        self.current_instruction = decode_instruction(&opcode);

        self.registers.pc.increment();
    }
}


#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;
    use crate::cpu::instructions::{NullableInstruction, NullableInstructionInternal};
    use super::*;


    #[test]
    fn executes_stored_instruction() {
        let nullable_internal = Rc::new(RefCell::new(NullableInstructionInternal::new()));
        let nullable_instruction = Box::new(NullableInstruction::new(nullable_internal.clone(), 0xDD, true));
        let memory = Arc::new(Mutex::new(MemoryController::new()));

        let mut cpu = GameBoyCPU::new(nullable_instruction);

        cpu.clock(memory.clone());

        assert_eq!(nullable_internal.borrow().was_executed, true);
    }

    #[test]
    fn gets_next_instruction_when_told_to_by_current_instruction() {
        let nullable_internal = Rc::new(RefCell::new(NullableInstructionInternal::new()));
        let nullable_instruction = Box::new(NullableInstruction::new(nullable_internal.clone(), 0xDD, true));
        let memory = Arc::new(Mutex::new(MemoryController::new()));

        let mut cpu = GameBoyCPU::new(nullable_instruction);

        cpu.clock(memory.clone());

        assert_eq!(0xFF, cpu.current_instruction.get_opcode()); //next instruction will be RST 38 in uninitialised ROM space
    }

    #[test]
    fn does_not_get_instruction_when_instruction_not_finished() {
        let nullable_internal = Rc::new(RefCell::new(NullableInstructionInternal::new()));
        let nullable_instruction = Box::new(NullableInstruction::new(nullable_internal.clone(), 0xDD, false));
        let memory = Arc::new(Mutex::new(MemoryController::new()));

        let mut cpu = GameBoyCPU::new(nullable_instruction);

        cpu.clock(memory.clone());

        assert_eq!(0xDD, cpu.current_instruction.get_opcode()); //next instruction will be RST 38 in uninitialised ROM space
    }

    #[test]
    fn get_next_instruction_increments_pc() {
        let nullable_internal = Rc::new(RefCell::new(NullableInstructionInternal::new()));
        let nullable_instruction = Box::new(NullableInstruction::new(nullable_internal.clone(), 0xDD, true));
        let memory = Arc::new(Mutex::new(MemoryController::new()));

        let mut cpu = GameBoyCPU::new(nullable_instruction);

        cpu.clock(memory.clone());

        assert_eq!(0x101, cpu.registers.pc.get_value()); //next instruction will be RST 38 in uninitialised ROM space
    }
}
