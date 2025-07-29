use crate::cpu::alu::ALU;
use crate::cpu::instructions::Instruction;
use crate::cpu::register::Register;
use crate::cpu::registers::Registers;
use crate::memory::{MemoryController, MemoryTrait};
use parking_lot::Mutex;
use std::sync::Arc;

//Timings for this one might be a bit off due to te 4-cycle alignment

pub struct LdNnSp {
    counter: u8,
    address: u16,
}

impl Instruction for LdNnSp {

    #[inline]
    fn from_opcode(opcode: &u8) -> Option<Box<dyn Instruction>> {
        if *opcode == 0x08 {
            return Some(Box::new(LdNnSp { counter: 4, address: 0 }))
        }
        None
    }

    fn get_opcode(&self) -> u8 {
        0x08
    }

    fn act(&mut self, registers: &mut Registers, _alu: &mut ALU, memory_controller: Arc<Mutex<MemoryController>>, _enable_interrupts: &mut bool, _is_halted: &mut bool) -> bool {
        if self.counter == 4 {
            self.address = memory_controller.lock().get(registers.pc.get_value()) as u16;
            registers.pc.increment();
        }
        else if self.counter == 3 {
            self.address = self.address | ((memory_controller.lock().get(registers.pc.get_value()) as u16) << 8);
            registers.pc.increment();
        }
        else if self.counter == 2 {
            memory_controller.lock().set(self.address, (registers.sp.get_value() & 0x00FF) as u8);
            self.address += 1;
        }
        else if self.counter == 1 {
            memory_controller.lock().set(self.address,  (registers.sp.get_value() >> 8) as u8);
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
    use super::*;
    use crate::cpu::register::Register;
    use crate::memory::MemoryTrait;

    reusable_testing_macro!(0x08, LdNnSp);

    #[test]
    fn load_address_on_tick_4() {
        let mut registers = Registers::new(0, 0, 0, 0, 0xC000, 0);
        let mut alu = ALU::new(registers.f.clone());
        let memory = Arc::new(Mutex::new(MemoryController::new()));

        memory.lock().set(0xC000, 0x12);

        let mut instruction = LdNnSp { counter: 4, address: 0 };

        let result = instruction.act(&mut registers, &mut alu, memory.clone(),&mut false, &mut false);

        assert_eq!(false, result);

        assert_eq!(0x12, instruction.address);
        assert_eq!(0xC001, registers.pc.get_value());
    }

    #[test]
    fn load_address_on_tick_3() {
        let mut registers = Registers::new(0, 0, 0, 0, 0xC001, 0);
        let mut alu = ALU::new(registers.f.clone());
        let memory = Arc::new(Mutex::new(MemoryController::new()));

        memory.lock().set(0xC001, 0xC0);

        let mut instruction = LdNnSp { counter: 3, address: 0x12 };

        let result = instruction.act(&mut registers, &mut alu, memory.clone(),&mut false, &mut false);

        assert_eq!(false, result);

        assert_eq!(0xC012, instruction.address);
        assert_eq!(0xC002, registers.pc.get_value());
    }

    #[test]
    fn load_sp_low_on_tick_2() {
        let expected_a_value: u8 = 0x12;

        let mut registers = Registers::new(0, 0, 0, 0, 0xC002, 0);
        registers.sp.set_value(0xFF00 | expected_a_value as u16);

        let mut alu = ALU::new(registers.f.clone());
        let memory = Arc::new(Mutex::new(MemoryController::new()));

        memory.lock().set(0xC012, 0x00);

        let mut instruction = LdNnSp { counter: 2, address: 0xC012 };

        let result = instruction.act(&mut registers, &mut alu, memory.clone(),&mut false, &mut false);

        assert_eq!(false, result);
        assert_eq!(expected_a_value, memory.lock().get(0xC012));
        assert_eq!(0xC002, registers.pc.get_value());
        assert_eq!(0xC013, instruction.address);
    }

    #[test]
    fn load_sp_high_on_tick_1() {
        let expected_a_value = 0x12;

        let mut registers = Registers::new(0, 0, 0, 0, 0xC002, 0);
        registers.sp.set_value(((expected_a_value as u16) << 8) | 0xFF);

        let mut alu = ALU::new(registers.f.clone());
        let memory = Arc::new(Mutex::new(MemoryController::new()));

        memory.lock().set(0xC013, 0x00);

        let mut instruction = LdNnSp { counter: 1, address: 0xC013 };

        let result = instruction.act(&mut registers, &mut alu, memory.clone(),&mut false, &mut false);

        assert_eq!(false, result);
        assert_eq!(expected_a_value, memory.lock().get(0xC013));
        assert_eq!(0xC002, registers.pc.get_value());
    }

    #[test]
    fn get_next_instruction_on_tick_0() {
        let mut registers = Registers::new(0, 0, 0, 0, 0xC002, 0);
        let mut alu = ALU::new(registers.f.clone());
        let memory = Arc::new(Mutex::new(MemoryController::new()));

        memory.lock().set(0xC000, 0x12);

        let mut instruction = LdNnSp { counter: 0, address: 0x1234 };

        let result = instruction.act(&mut registers, &mut alu, memory.clone(),&mut false, &mut false);

        assert_eq!(true, result);
        assert_eq!(0xC002, registers.pc.get_value());
    }
}
