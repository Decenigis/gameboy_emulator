use crate::cpu::registers::Registers;

pub struct CPU {
    registers: Registers
}


impl CPU {
    pub fn new() -> Self {
        Self {
            registers: Registers::new(
                0,
                0,
                0,
                0,
                0,
                0
            )
        }
    }
}
