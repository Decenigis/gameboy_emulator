use std::cmp::min;
use std::fs;
use std::path::Path;
use dialog::{DialogBox, Message};
use crate::memory::MemoryTrait;

pub struct ROM {
    data: Vec<Vec<u8>>,
    active_bank: usize
}

impl MemoryTrait for ROM {
    fn get(&self, position: u16) -> u8 {
        match self.get_rom_ref(position) {
            Some(byte) => {
                *byte
            }
            None => 0xFF
        }
    }

    fn set(&mut self, position: u16, value: u8) -> u8 {
        if position >= 0x2000 && position < 0x4000 {
            self.active_bank = value as usize;
        }

        match self.get_rom_ref(position) {
            Some(byte) => {
                *byte
            }
            None => 0xFF
        }
    }

    fn has_address(&self, position: u16) -> bool {
        position < 2 * Self::ROM_BANK_SIZE as u16
    }
}

impl ROM {

    const ROM_BANK_SIZE: usize = 0x4000;

    pub fn new() -> Self {
        Self {
            data: vec![vec![0; Self::ROM_BANK_SIZE], vec![0; Self::ROM_BANK_SIZE]],
            active_bank: 1
        }
    }

    pub fn load_rom_file(&mut self, path: &Path) {
        self.data.clear();

        let mut file_data = match fs::read(path) {
            Ok(data) => data,
            Err(_) => {
                let _ = Message::new(format!("Could not open file: {}", path.to_str().unwrap())).title("File Error").show();

                self.data = vec![vec![0; Self::ROM_BANK_SIZE], vec![0; Self::ROM_BANK_SIZE]];
                return
            }
        };

        while file_data.len() > 0 {
            let remaining_bank_size = min(file_data.len(), Self::ROM_BANK_SIZE);
            let current_bank = file_data[0..remaining_bank_size].to_vec();

            self.data.push(current_bank);

            file_data = file_data[remaining_bank_size..file_data.len()].to_vec();
        }

        return;
    }

    fn get_rom_ref(&self, position: u16) -> Option<&u8> {
        let bank_position = position as usize % Self::ROM_BANK_SIZE;
        let relevant_bank = self.get_relevant_bank(position);

        if bank_position < self.data[relevant_bank].len() {
            Some(&self.data[relevant_bank][bank_position])
        } else {
            None
        }
    }

    fn get_relevant_bank(&self, position: u16) -> usize {
        if position < Self::ROM_BANK_SIZE as u16 {
            0
        } else {
            self.active_bank % self.data.len() //currently simulates MBC5
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rom_gets_from_bank_0_correctly() {
        let expected_value = 0x12;
        let mut rom = ROM::new();

        rom.data[0][0x0123] = expected_value;

        assert_eq!(rom.get(0x0123), expected_value);
    }

    #[test]
    fn rom_sets_to_bank_1_correctly() {
        let expected_value = 0x12;
        let mut rom = ROM::new();

        rom.data[1][0x0123] = expected_value;

        assert_eq!(rom.get(0x4123), expected_value);
    }

    #[test]
    fn sets_to_other_banks_correctly() {
        let expected_value = 0x12;
        let mut rom = ROM::new();

        rom.data.push(vec![0; ROM::ROM_BANK_SIZE]);
        rom.data.push(vec![0; ROM::ROM_BANK_SIZE]); //Pretend to be a 64KB ROM

        rom.set(0x2000, 0x02);
        rom.data[2][0x0123] = expected_value;

        assert_eq!(rom.get(0x4123), expected_value);
    }

    //might be hard to test rom loading
}
