use crate::memory::MemoryTrait;

pub struct Divider {
    clock: u16,

    memory_value: u8
}

impl MemoryTrait for Divider {
    fn get(&self, position: u16) -> u8 {
        if position == 0xFF04 {
            self.memory_value
        } else {
            0xFF
        }
    }

    fn set(&mut self, position: u16, _value: u8) -> u8 {
        if position == 0xFF04 {
            self.memory_value = 0;
            0xFF
        } else {
            0xFF
        }
    }

    fn has_address(&self, position: u16) -> bool {
        position == 0xFF04
    }
}


impl Divider {
    pub fn new() -> Self {
        Self {
            clock: 0,
            memory_value: 0xAB
        }
    }

    pub fn clock(&mut self) {
        self.clock += 1;

        if self.clock >= 512 { // Increment every 64 ticks
            self.clock = 0;
            self.memory_value = self.memory_value.wrapping_add(1);
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::memory::MemoryTrait;

    #[test]
    fn returns_value_when_ff04() {
        let divider = super::Divider::new();
        assert_eq!(divider.get(0xFF04), 0xAB);
    }

    #[test]
    fn resets_value_when_ff04_set() {
        let mut divider = super::Divider::new();

        divider.set(0xFF04, 0x12);
        assert_eq!(divider.get(0xFF04), 0x00);
    }
    
    #[test]
    fn does_not_return_when_not_ff04() {
        let divider = super::Divider::new();
        assert_eq!(divider.get(0xFF05), 0xFF);
    }
    
    #[test]
    fn does_not_set_when_not_ff04() {
        let mut divider = super::Divider::new();
        
        assert_eq!(divider.set(0xFF05, 0x12), 0xFF);
        assert_eq!(divider.get(0xFF04), 0xAB);
    }
    
    #[test]
    fn increments_value_on_clock() {
        let mut divider = super::Divider::new();
        
        for _ in 0..512 {
            divider.clock();
        }
        
        assert_eq!(divider.get(0xFF04), 0xAC);
    }
}
