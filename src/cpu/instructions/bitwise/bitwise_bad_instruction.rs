use crate::cpu::alu::ALU;
use crate::cpu::instructions::instruction::Instruction;
use crate::cpu::register::Register;
use crate::cpu::registers::Registers;
use crate::memory::MemoryController;
use parking_lot::Mutex;
use std::sync::Arc;

pub struct BitwiseBadInstruction {

    opcode: u8
}

impl Instruction for BitwiseBadInstruction {

    #[inline]
    fn from_opcode(opcode: &u8) -> Option<Box<dyn Instruction>> {
        Some(Box::new(Self {opcode: *opcode}))
    }

    fn get_opcode(&self) -> u8 {
        self.opcode
    }

    fn act(&mut self, registers: &mut Registers, _alu: &mut ALU, _memory_controller: Arc<Mutex<MemoryController>>, _enable_interrupts: &mut bool, _is_halted: &mut bool) -> bool {
        println!("Bad instruction '0xCB, {:#X}' at address '{:#X}'", self.opcode, registers.pc.get_value().wrapping_sub(1));

        // let mut file = File::create("memdump.bin").unwrap();
        // let mut memory: Vec<u8> = vec![];
        //
        // for i in 0..0xFFFF {
        //     memory.push(memory_controller.lock().get(i));
        // }

        //file.write_all(memory.as_slice()).unwrap();

        false
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_opcode_returns_always() {
        let instruction1 = BitwiseBadInstruction::from_opcode(&0x00);
        let instruction2 = BitwiseBadInstruction::from_opcode(&0xDD);

        assert_eq!(true, instruction1.is_some());
        assert_eq!(true, instruction2.is_some());
    }

    #[test]
    fn get_opcode_returns_given_opcode() {
        let opcode = 0xDD;
        let instruction = BitwiseBadInstruction::from_opcode(&opcode).unwrap();

        assert_eq!(opcode, instruction.get_opcode());
    }

    #[test]
    fn act_immediately_returns_false() {
        let mut registers = Registers::new(0, 0, 0, 0, 0, 0);
        let mut alu = ALU::new(registers.f.clone());

        let mut instruction = BitwiseBadInstruction { opcode: 0x00 };

        let result = instruction.act(&mut registers, &mut alu, Arc::new(Mutex::new(MemoryController::new())), &mut false, &mut false);

        assert_eq!(false, result);
    }
}
