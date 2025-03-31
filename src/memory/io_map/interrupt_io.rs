use crate::memory::MemoryTrait;

pub struct InterruptIO {
    pub interrupt_flag: u8, // Interrupt Flag Register
    pub interrupt_enable: u8, // Interrupt Enable Register
}

impl MemoryTrait for InterruptIO {
    fn get(&self, address: u16) -> u8 {
        match address {
            0xFF0F => self.interrupt_flag,
            0xFFFF => self.interrupt_enable,
            _ => panic!("Invalid address for InterruptIO: {:#X}", address),
        }
    }

    fn set(&mut self, address: u16, value: u8) -> u8 {
        match address {
            0xFF0F => {
                let old_value = self.interrupt_flag;
                self.interrupt_flag = value;
                old_value
            },
            0xFFFF => {
                let old_value = self.interrupt_enable;
                self.interrupt_enable = value;
                old_value
            }
            _ => 0xFF,
        }
    }

    fn has_address(&self, position: u16) -> bool {
        position == 0xFF0F || position == 0xFFFF
    }
}

impl InterruptIO {
    pub fn new() -> Self {
        Self {
            interrupt_flag: 0x00,
            interrupt_enable: 0x00,
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_set_interrupt_flag() {
        let mut interrupt_io = InterruptIO::new();

        interrupt_io.set(0xFF0F, 0xAB);
        assert_eq!(interrupt_io.get(0xFF0F), 0xAB);
    }

    #[test]
    fn test_get_set_interrupt_enable() {
        let mut interrupt_io = InterruptIO::new();

        interrupt_io.set(0xFFFF, 0xCD);
        assert_eq!(interrupt_io.get(0xFFFF), 0xCD);
    }
}
