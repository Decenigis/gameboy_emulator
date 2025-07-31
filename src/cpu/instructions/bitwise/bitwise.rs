use std::sync::Arc;
use parking_lot::Mutex;
use crate::{bit_b_hl_decode_instruction, bit_b_r_decode_instruction, res_b_hl_decode_instruction, res_b_r_decode_instruction, rr_r_decode_instruction, set_b_hl_decode_instruction, set_b_r_decode_instruction, srl_r_decode_instruction, swap_r_decode_instruction};
use crate::cpu::alu::ALU;
use crate::cpu::instructions::bitwise::bitwise_bad_instruction::BitwiseBadInstruction;
use crate::cpu::instructions::Instruction;
use crate::cpu::register::Register;
use crate::cpu::registers::Registers;
use crate::memory::{MemoryController, MemoryTrait};
pub use super::*;

pub struct Bitwise {
    instruction: Option<Box<dyn Instruction>>,
}

impl Bitwise {
    pub fn decode_instruction(opcode: &u8) -> Box<dyn Instruction> {
        rr_r_decode_instruction!(opcode);
        srl_r_decode_instruction!(opcode);
        bit_b_r_decode_instruction!(opcode);
        bit_b_hl_decode_instruction!(opcode);
        res_b_r_decode_instruction!(opcode);
        res_b_hl_decode_instruction!(opcode);
        set_b_r_decode_instruction!(opcode);
        set_b_hl_decode_instruction!(opcode);
        swap_r_decode_instruction!(opcode);

        BitwiseBadInstruction::from_opcode(opcode).unwrap()
    }
}


impl Instruction for Bitwise {
    fn from_opcode(opcode: &u8) -> Option<Box<dyn Instruction>>
    where
        Self: Sized
    {
        if *opcode == 0xCB {
            return Some(Box::new(Bitwise { instruction: None }));
        }
        None
    }

    fn get_opcode(&self) -> u8 {
        0xCB
    }

    fn act(&mut self, registers: &mut Registers, alu: &mut ALU, memory_controller: Arc<Mutex<MemoryController>>, enable_interrupts: &mut bool, is_halted: &mut bool) -> bool {
        match self.instruction {
            Some(ref mut instruction) => {
                instruction.act(registers, alu, memory_controller, enable_interrupts, is_halted)
            }
            None => {
                let next_opcode = memory_controller.lock().get(registers.pc.get_value());
                self.instruction = Some(Bitwise::decode_instruction(&next_opcode));

                registers.pc.increment();

                self.act(registers, alu, memory_controller, enable_interrupts, is_halted)
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;
    use crate::cpu::instructions::{NullableInstruction, NullableInstructionInternal};
    use super::*;

    reusable_testing_macro!(0xCB, Bitwise);

    #[test]
    fn decode_instruction_loads_an_instruction() {
        let mut registers = Registers::new(0, 0, 0, 0, 0xC000, 0);
        let memory = Arc::new(Mutex::new(MemoryController::new()));
        let mut alu = ALU::new(registers.f.clone());

        memory.lock().set(0xC000, 0xCB);
        memory.lock().set(0xC001, 0x7F); // Example opcode for BIT 7, A

        let mut instruction = Bitwise { instruction: None };
        assert_eq!(false, instruction.act(&mut registers, &mut alu, memory.clone(), &mut false, &mut false));

        assert_eq!(true, instruction.instruction.is_some());
    }

    #[test]
    fn decode_instruction_acts_on_next_instruction() {
        let mut registers = Registers::new(0, 0, 0, 0, 0xC000, 0);
        let memory = Arc::new(Mutex::new(MemoryController::new()));
        let mut alu = ALU::new(registers.f.clone());

        let nullable_instruction_internal = Rc::new(RefCell::new(NullableInstructionInternal::new()));

        let mut instruction = Bitwise { instruction: Some(Box::new(NullableInstruction::new(nullable_instruction_internal.clone(), 0x00, false))) };

        instruction.act(&mut registers, &mut alu, memory.clone(), &mut false, &mut false);

        assert_eq!(true, nullable_instruction_internal.borrow().was_executed);
    }

}
