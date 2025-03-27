use crate::memory::MemoryTrait;

pub struct RAM { //also echo RAM
    data: Vec<Vec<u8>>,
    active_bank: usize
}

impl MemoryTrait for RAM {

    fn get(&self, position: u16) -> u8 {
        match self.get_bank_ref(position) {
            Some(bank) => bank[position as usize % Self::RAM_BANK_SIZE],
            None => 0xFF
        }
    }

    fn set(&mut self, position: u16, value: u8) -> u8 {
        match self.get_bank_ref_mut(position) {
            Some(bank) => {
                let old_value = bank[position as usize % Self::RAM_BANK_SIZE];
                bank[position as usize % Self::RAM_BANK_SIZE] = value;
                old_value
            }
            None => 0xFF
        }
    }

    fn has_address(&self, position: u16) -> bool {
        position >= Self::RAM_LOWER_BOUND && position < Self::ECHO_RAM_UPPER_BOUND
    }
}


impl RAM {

    const RAM_BANK_SIZE: usize = 0x1000;
    const RAM_LOWER_BOUND: u16 = 0xC000;
    const ECHO_RAM_UPPER_BOUND: u16 = 0xFE00;

    pub fn new() -> Self {
        Self {
            data: vec![vec![0; Self::RAM_BANK_SIZE]; 8], //32KB ready for GBC support
            active_bank: 1 //always 1 until GBC support is added. May be driven by the IO map directly in future but this could cause mutex shenanigans
        }
    }

    fn get_bank_ref(&self, position: u16) -> Option<&Vec<u8>> {
        if position < Self::RAM_LOWER_BOUND || position >= Self::ECHO_RAM_UPPER_BOUND {
            None
        }
        else if position % 0x2000 < 0x1000 {
            Some(&self.data[0])
        }
        else {
            let active_bank_real = if self.active_bank == 0 { 1 } else { self.active_bank };
            Some(&self.data[active_bank_real])
        }
    }

    fn get_bank_ref_mut(&mut self, position: u16) -> Option<&mut Vec<u8>> {
        if position < Self::RAM_LOWER_BOUND || position >= Self::ECHO_RAM_UPPER_BOUND {
            None
        }
        else if position % 0x2000 < 0x1000 {
            Some(&mut self.data[0])
        }
        else {
            let active_bank_real = if self.active_bank == 0 { 1 } else { self.active_bank };
            Some(&mut self.data[active_bank_real])
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ram_sets_to_low_bank_correctly() {
        let expected_value = 0x12;
        let mut ram = RAM::new();

        ram.set(0xC000, expected_value);

        assert_eq!(ram.data[0][0], 0x12);
    }

    #[test]
    fn ram_sets_to_low_bank_in_echo_ram_correctly() {
        let expected_value = 0x12;
        let mut ram = RAM::new();

        ram.set(0xE000, expected_value);

        assert_eq!(ram.data[0][0], expected_value);
    }


    #[test]
    fn ram_gets_from_low_bank_correctly() {
        let expected_value = 0x12;
        let mut ram = RAM::new();

        ram.data[0][0] = expected_value;

        assert_eq!(ram.get(0xC000), expected_value);
    }

    #[test]
    fn ram_gets_from_low_bank_in_echo_ram_correctly() {
        let expected_value = 0x12;
        let mut ram = RAM::new();

        ram.data[0][0] = expected_value;

        assert_eq!(ram.get(0xE000), expected_value);
    }
    #[test]
    fn ram_sets_to_high_bank_correctly() {
        let expected_value = 0x12;
        let mut ram = RAM::new();

        ram.set(0xD000, expected_value);

        assert_eq!(ram.data[1][0], 0x12);
    }

    #[test]
    fn ram_sets_to_low_high_in_echo_ram_correctly() {
        let expected_value = 0x12;
        let mut ram = RAM::new();

        ram.set(0xF000, expected_value);

        assert_eq!(ram.data[1][0], expected_value);
    }


    #[test]
    fn ram_gets_from_high_bank_correctly() {
        let expected_value = 0x12;
        let mut ram = RAM::new();

        ram.data[1][0] = expected_value;

        assert_eq!(ram.get(0xD000), expected_value);
    }

    #[test]
    fn ram_gets_from_high_bank_in_echo_ram_correctly() {
        let expected_value = 0x12;
        let mut ram = RAM::new();

        ram.data[1][0] = expected_value;

        assert_eq!(ram.get(0xF000), expected_value);
    }



    #[test]
    fn echo_ram_does_not_interfere_with_oam() {
        let expected_value = 0xFF;
        let mut ram = RAM::new();

        ram.set(0xDE00, 0x12);

        assert_eq!(ram.get(0xFE00), expected_value);
    }


    #[test]
    fn has_address_in_bounds_is_true() {
        let ram = RAM::new();

        assert!(ram.has_address(0xC000));
    }

    #[test]
    fn has_address_low_out_of_bounds_is_false() {
        let ram = RAM::new();

        assert!(!ram.has_address(0xBFFF));
    }

    #[test]
    fn has_address_high_out_of_bounds_is_false() {
        let ram = RAM::new();

        assert!(!ram.has_address(0xFE00));
    }
}
