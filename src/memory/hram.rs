use crate::memory::MemoryTrait;

pub struct HRAM {
    data: Vec<u8>,
}


impl MemoryTrait for HRAM {
    fn get(&self, position: u16) -> u8 {
        self.data[position as usize % Self::HRAM_SIZE]
    }

    fn set(&mut self, position: u16, value: u8) -> u8 {
        let old_value = self.data[position as usize % Self::HRAM_SIZE];
        self.data[position as usize % Self::HRAM_SIZE] = value;
        old_value
    }

    fn has_address(&self, position: u16) -> bool {
        position >= 0xFF80 && position < 0xFFFF
    }
}

impl HRAM {

    const HRAM_SIZE: usize = 0x80;

    pub fn new() -> Self {
        Self { data: vec![0; Self::HRAM_SIZE] }
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_hram_stores_data() {
        let mut hram = HRAM::new();

        hram.set(0xFF80, 0x12);

        assert_eq!(hram.get(0xFF80), 0x12);
    }

    #[test]
    fn has_address_in_bounds_is_true() {
        let hram = HRAM::new();

        assert!(hram.has_address(0xFF90));
    }

    #[test]
    fn has_address_low_out_of_bounds_is_false() {
        let hram = HRAM::new();

        assert!(!hram.has_address(0xFF7F));
    }

    #[test]
    fn has_address_high_out_of_bounds_is_false() {
        let hram = HRAM::new();

        assert!(!hram.has_address(0xFFFF));
    }
}