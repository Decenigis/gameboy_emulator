use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;
use parking_lot::Mutex;
use crate::cpu::alu::ALU;
use crate::cpu::instructions::Instruction;
use crate::cpu::registers::Registers;
use crate::memory::MemoryController;

pub struct NullableInstructionInternal {
    pub was_executed: bool,
}


impl NullableInstructionInternal {
    pub fn new() -> Self {
        Self {
            was_executed: false,
        }
    }
}


pub struct NullableInstruction {
    internal: Rc<RefCell<NullableInstructionInternal>>,
    opcode: u8,
    get_next_instruction: bool
}

impl NullableInstruction {
    pub fn new(internal: Rc<RefCell<NullableInstructionInternal>>, opcode: u8, get_next_instruction: bool) -> Self {
        Self {
            internal,
            opcode,
            get_next_instruction,
        }
    }
}

impl Instruction for NullableInstruction {
    fn from_opcode(_opcode: &u8) -> Option<Box<dyn Instruction>> {
        todo!()
    }

    fn get_opcode(&self) -> u8 {
        self.opcode
    }

    fn act(&mut self, _registers: &mut Registers, _alu: &mut ALU, _memory_controller: Arc<Mutex<MemoryController>>) -> bool {
        self.internal.borrow_mut().was_executed = true;

        self.get_next_instruction
    }
}
