use std::sync::Arc;
use parking_lot::Mutex;
use crate::cpu::alu::ALU;
use crate::cpu::instructions::Instruction;
use crate::cpu::register16::Register16;
use crate::cpu::register8::Register8;
use crate::cpu::register::Register;
use crate::cpu::registers::Registers;
use crate::memory::{MemoryController, MemoryTrait};

pub struct ReusableLdRN {
    counter: u8
}

impl ReusableLdRN {
    pub(crate) fn new() -> Self {
        ReusableLdRN { counter: 1 }
    }

    pub(crate) fn act(&mut self, register: &mut Register8, pc: &mut Register16, memory_controller: Arc<Mutex<MemoryController>>) -> bool {
        if self.counter == 1 {
            register.set_value(memory_controller.lock().get(pc.get_value()));
            pc.increment();
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

    #[test]
    fn load_n_into_a_on_tick_1() {
        let mut register = Register8::new(0);
        let mut pc = Register16::new(0xC000);
        let memory = Arc::new(Mutex::new(MemoryController::new()));

        memory.lock().set(0xC000, 0x12);

        let mut instruction = ReusableLdRN { counter: 1 };

        let result = instruction.act(&mut register, &mut pc, memory.clone());

        assert_eq!(false, result);
        assert_eq!(0x12, register.get_value());
        assert_eq!(0xC001, pc.get_value());
    }

    #[test]
    fn on_tick_0_get_next_instruction() {
        let mut register = Register8::new(0);
        let mut pc = Register16::new(0xC001);
        let memory = Arc::new(Mutex::new(MemoryController::new()));

        let mut instruction = ReusableLdRN { counter: 0 };

        let result = instruction.act(&mut register, &mut pc, memory.clone());

        assert_eq!(true, result);
        assert_eq!(0x00, register.get_value());
        assert_eq!(0xC001, pc.get_value());
    }
}
