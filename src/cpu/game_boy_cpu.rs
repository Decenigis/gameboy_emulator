use std::collections::VecDeque;
use std::fs;
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
    is_halted: bool,
    current_instruction: Box<dyn Instruction>,
    interrupt: Option<Interrupt>,

    callstack: VecDeque<String>,
}

impl Drop for GameBoyCPU {
    fn drop(&mut self) {
        let mut a = String::new();

        for val in self.callstack.iter() {
            a.push_str(val.as_str());
        }

        fs::write("./callstack", a);
    }
}


impl CPU for GameBoyCPU {

    fn clock (&mut self, memory: Arc<Mutex<MemoryController>>) {
        if self.is_halted {
            return;
        }

        let instruction_finished = self.current_instruction.act(&mut self.registers, &mut self.alu, memory.clone(), &mut self.enable_interrupts, &mut self.is_halted);

        if instruction_finished {
            self.load_next_instruction(memory)
        }
    }

    fn try_interrupt(&mut self, memory: Arc<Mutex<MemoryController>>, interrupt: Interrupt) {
        if !self.enable_interrupts {
            return;
        }

        let ie_ = memory.lock().get(0xFFFF);
        let if_ = memory.lock().get(0xFF0F);

        if (ie_ & interrupt.get_bit_mask()) == 0 {
            return;
        }

        self.interrupt = Some(interrupt);
        self.is_halted = false;

        memory.lock().set(0xFF0F, if_ | interrupt.get_bit_mask());
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
            0xFFFE
        );
        let f = registers.f.clone();

        Self {
            registers,
            alu: ALU::new(f),
            enable_interrupts: false,
            is_halted: false,
            current_instruction: first_instruction,
            interrupt: None,

            callstack: VecDeque::new()
        }
    }


    fn load_next_instruction (&mut self, memory: Arc<Mutex<MemoryController>>) {
        match self.interrupt { //this prevents interrupting mid-instruction
            Some(interrupt) => {
                self.interrupt = None;

                self.registers.sp.decrement();
                memory.lock().set(self.registers.sp.get_value(), ((self.registers.pc.get_value() & 0xFF00) >> 8) as u8);
                self.registers.sp.decrement();
                memory.lock().set(self.registers.sp.get_value(), (self.registers.pc.get_value() & 0xFF) as u8);

                self.registers.pc.set_value(interrupt.get_address());
            }
            None => {}
        }

        self.callstack.push_back(format!("{:02X}: {:04X}\n", memory.lock().get_rom().get_relevant_bank(self.registers.pc.get_value()), self.registers.pc.get_value()));

        if self.callstack.len() > 1000000 {
            self.callstack.pop_front();
        }

        if self.registers.pc.get_value() == 0x5b4B {
            //panic!("{}", self.registers);
        }

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
    fn does_not_execute_when_halted() {
        let nullable_internal = Rc::new(RefCell::new(NullableInstructionInternal::new()));
        let nullable_instruction = Box::new(NullableInstruction::new(nullable_internal.clone(), 0xDD, true));
        let memory = Arc::new(Mutex::new(MemoryController::new()));

        let mut cpu = GameBoyCPU::new(nullable_instruction);
        cpu.is_halted = true;

        cpu.clock(memory.clone());

        assert_eq!(nullable_internal.borrow().was_executed, false);
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

    #[test]
    fn interrupt_sets_address_properly() {
        let memory = Arc::new(Mutex::new(MemoryController::new()));
        memory.lock().set(0xFFFF, 0x01);
        memory.lock().set(0xFF0F, 0x00);

        let mut cpu = GameBoyCPU::new_with_nop();

        cpu.enable_interrupts = true;

        cpu.try_interrupt(memory.clone(), Interrupt::VBlank);
        cpu.clock(memory.clone());

        assert_eq!(0x0041, cpu.registers.pc.get_value());
    }

    #[test]
    fn interrupt_pushes_to_stack() {
        let memory = Arc::new(Mutex::new(MemoryController::new()));
        memory.lock().set(0xFFFF, 0x01);
        memory.lock().set(0xFF0F, 0x00);

        let mut cpu = GameBoyCPU::new_with_nop();
        cpu.registers.sp.set_value(0xD000);
        cpu.registers.pc.set_value(0x1234);

        cpu.enable_interrupts = true;

        cpu.try_interrupt(memory.clone(), Interrupt::VBlank);
        cpu.clock(memory.clone());

        assert_eq!(0xCFFE, cpu.registers.sp.get_value());
        assert_eq!(0x34, memory.lock().get(0xCFFE));
        assert_eq!(0x12, memory.lock().get(0xCFFF));
    }

    #[test]
    fn unhalts_when_interrupted() {
        let memory = Arc::new(Mutex::new(MemoryController::new()));
        memory.lock().set(0xFFFF, 0x01);
        memory.lock().set(0xFF0F, 0x00);

        let mut cpu = GameBoyCPU::new_with_nop();

        cpu.enable_interrupts = true;

        cpu.try_interrupt(memory.clone(), Interrupt::VBlank);
        cpu.clock(memory.clone());

        assert_eq!(false, cpu.is_halted);
    }

    #[test]
    fn does_not_interrupt_when_interrupts_disabled() {
        let expected_pc = 0x100;
        let memory = Arc::new(Mutex::new(MemoryController::new()));
        memory.lock().set(0xFFFF, 0x01);
        memory.lock().set(0xFF0F, 0x00);
        let mut cpu = GameBoyCPU::new_with_nop();
        cpu.registers.pc.set_value(expected_pc);
        cpu.enable_interrupts = false;

        cpu.try_interrupt(memory.clone(), Interrupt::VBlank);

        assert_eq!(expected_pc, cpu.registers.pc.get_value());
    }

    #[test]
    fn does_not_interrupt_when_ie_not_set() {
        let expected_pc = 0x100;
        let memory = Arc::new(Mutex::new(MemoryController::new()));
        memory.lock().set(0xFFFF, 0x00);
        memory.lock().set(0xFF0F, 0x00);
        let mut cpu = GameBoyCPU::new_with_nop();
        cpu.registers.pc.set_value(expected_pc);
        cpu.enable_interrupts = true;

        cpu.try_interrupt(memory.clone(), Interrupt::VBlank);

        assert_eq!(expected_pc, cpu.registers.pc.get_value());
    }
}
