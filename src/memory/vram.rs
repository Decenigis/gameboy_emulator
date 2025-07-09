use std::fs;
use std::path::Path;
use crate::memory::memory_trait::MemoryTrait;

pub struct VRAM {
    tile_bank_0: Vec<u8>, tile_bank_0_stale: bool,
    tile_bank_1: Vec<u8>, tile_bank_1_stale: bool,
    tile_bank_2: Vec<u8>, tile_bank_2_stale: bool,

    map_bank_0: Vec<u8>, map_bank_0_stale: bool,
    map_bank_1: Vec<u8>, map_bank_1_stale: bool
}

/*
    0x8000-0x87FF -> Tile bank 0
    0x8800-0x8FFF -> Tile bank 1
    0x9000-0x97FF -> Tile bank 2

    0x9800-0x9BFF -> TileMap bank 0
    0x9C00-0x9FFF -> TileMap bank 1
 */

impl MemoryTrait for VRAM {

    fn get(&self, position: u16) -> u8 {

        let position_in_tile_bank = position as usize % Self::TILE_BANK_SIZE;
        let position_in_map_bank = position as usize % Self::MAP_BANK_SIZE;

        if position < 0x8000 {
            0xFF
        }
        else if position < 0x8800 {
            self.tile_bank_0[position_in_tile_bank]
        }
        else if position < 0x9000 {
            self.tile_bank_1[position_in_tile_bank]
        }
        else if position < 0x9800 {
            self.tile_bank_2[position_in_tile_bank]
        }
        else if position < 0x9C00 {
            self.map_bank_0[position_in_map_bank]
        }
        else if position < 0xA000 {
            self.map_bank_1[position_in_map_bank]
        }
        else {
            0xFF
        }
    }

    fn set(&mut self, position: u16, value: u8) -> u8 {
        let old_value: u8;

        let position_in_tile_bank = position as usize % Self::TILE_BANK_SIZE;
        let position_in_map_bank = position as usize % Self::MAP_BANK_SIZE;

        if position < 0x8000 {
            old_value = 0xFF
        }
        else if position < 0x8800 {
            old_value = self.tile_bank_0[position_in_tile_bank];
            self.tile_bank_0[position_in_tile_bank] = value;
            self.tile_bank_0_stale = true;
        }
        else if position < 0x9000 {
            old_value = self.tile_bank_1[position_in_tile_bank];
            self.tile_bank_1[position_in_tile_bank] = value;
            self.tile_bank_1_stale = true;
        }
        else if position < 0x9800 {
            old_value = self.tile_bank_2[position_in_tile_bank];
            self.tile_bank_2[position_in_tile_bank] = value;
            self.tile_bank_2_stale = true;
        }
        else if position < 0x9C00 {
            old_value = self.map_bank_0[position_in_map_bank];
            self.map_bank_0[position_in_map_bank] = value;
            self.map_bank_0_stale = true;
        }
        else if position < 0xA000 {
            old_value = self.map_bank_1[position_in_map_bank];
            self.map_bank_1[position_in_map_bank] = value;
            self.map_bank_1_stale = true;
        }
        else {
            old_value = 0;
        }

        old_value
    }

    fn has_address(&self, position: u16) -> bool {
        position >= 0x8000 && position < 0xA000
    }
}

impl VRAM {

    const TILE_BANK_SIZE: usize = 0x800;
    const MAP_BANK_SIZE: usize = 0x400;

    pub fn new() -> Self {
        let mut tile_bank_0 = vec![];
        for byte in fs::read(Path::new("assets/graphics/initial_tile_data.bin")).unwrap() {
            tile_bank_0.push(byte);
        }
        tile_bank_0.resize(Self::TILE_BANK_SIZE, 0);

        let mut map_bank_0 = vec![0; 0x100];
        for byte in fs::read(Path::new("assets/graphics/initial_map_data.bin")).unwrap() {
            map_bank_0.push(byte);
        }
        map_bank_0.resize(Self::MAP_BANK_SIZE, 0);
        
        Self {
            tile_bank_0, tile_bank_0_stale: true,
            tile_bank_1: vec![0; Self::TILE_BANK_SIZE], tile_bank_1_stale: true,
            tile_bank_2: vec![0; Self::TILE_BANK_SIZE], tile_bank_2_stale: true,

            map_bank_0, map_bank_0_stale: true,
            map_bank_1: vec![0; Self::MAP_BANK_SIZE], map_bank_1_stale: true
        }
    }

    pub fn get_tile_bank_0_if_stale(&self) -> Option<&Vec<u8>> {
        if self.tile_bank_0_stale {
            Some(&self.tile_bank_0)
        } else {
            None
        }
    }

    pub fn get_tile_bank_1_if_stale(&self) -> Option<&Vec<u8>> {
        if self.tile_bank_1_stale {
            Some(&self.tile_bank_1)
        } else {
            None
        }
    }

    pub fn get_tile_bank_2_if_stale(&self) -> Option<&Vec<u8>> {
        if self.tile_bank_2_stale {
            Some(&self.tile_bank_2)
        } else {
            None
        }
    }

    pub fn get_map_bank_0_if_stale(&self) -> Option<&Vec<u8>> {
        if self.map_bank_0_stale {
            Some(&self.map_bank_0)
        } else {
            None
        }
    }

    pub fn get_map_bank_1_if_stale(&self) -> Option<&Vec<u8>> {
        if self.map_bank_1_stale {
            Some(&self.map_bank_1)
        } else {
            None
        }
    }

    pub fn set_not_stale(&mut self) {
        self.tile_bank_0_stale = false;
        self.tile_bank_1_stale = false;
        self.tile_bank_2_stale = false;
        self.map_bank_0_stale = false;
        self.map_bank_1_stale = false;
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tile_bank_0_is_correctly_written_to() {
        let mut vram = VRAM::new();

        vram.set(0x8000, 0x12);

        assert_eq!(vram.get(0x8000), 0x12);
    }

    #[test]
    fn tile_bank_1_is_correctly_written_to() {
        let mut vram = VRAM::new();

        vram.set(0x8800, 0x34);

        assert_eq!(vram.get(0x8800), 0x34);
    }

    #[test]
    fn tile_bank_2_is_correctly_written_to() {
        let mut vram = VRAM::new();

        vram.set(0x9000, 0x56);

        assert_eq!(vram.get(0x9000), 0x56);
    }

    #[test]
    fn map_bank_0_is_correctly_written_to() {
        let mut vram = VRAM::new();

        vram.set(0x9800, 0x78);

        assert_eq!(vram.get(0x9800), 0x78);
    }

    #[test]
    fn map_bank_1_is_correctly_written_to() {
        let mut vram = VRAM::new();

        vram.set(0x9C00, 0x9A);

        assert_eq!(vram.get(0x9C00), 0x9A);
    }

    #[test]
    fn tile_bank_0_correctly_updated() {
        let mut vram = VRAM::new();
        vram.set_not_stale();

        vram.set(0x8000, 0x12);

        let tile_bank_0 = vram.get_tile_bank_0_if_stale().unwrap();

        assert_eq!(tile_bank_0[0], 0x12);
    }

    #[test]
    fn get_tile_bank_1_correctly_updated() {
        let mut vram = VRAM::new();
        vram.set_not_stale();
        vram.set(0x8800, 0x34);

        let tile_bank_1 = vram.get_tile_bank_1_if_stale().unwrap();

        assert_eq!(tile_bank_1[0], 0x34);
    }

    #[test]
    fn get_tile_bank_2_correctly_updated() {
        let mut vram = VRAM::new();
        vram.set_not_stale();
        vram.set(0x9000, 0x56);

        let tile_bank_2 = vram.get_tile_bank_2_if_stale().unwrap();

        assert_eq!(tile_bank_2[0], 0x56);
    }

    #[test]
    fn get_map_bank_0_correctly_updated() {
        let mut vram = VRAM::new();
        vram.set_not_stale();
        vram.set(0x9800, 0x78);

        let map_bank_0 = vram.get_map_bank_0_if_stale().unwrap();

        assert_eq!(map_bank_0[0], 0x78);
    }

    #[test]
    fn get_map_bank_1_correctly_updated() {
        let mut vram = VRAM::new();
        vram.set_not_stale();
        vram.set(0x9C00, 0x9A);

        let map_bank_1 = vram.get_map_bank_1_if_stale().unwrap();

        assert_eq!(map_bank_1[0], 0x9A);
    }

    #[test]
    fn set_not_stale_sets_all_stale_flags_to_false() {
        let mut vram = VRAM::new();

        vram.set_not_stale();

        assert_eq!(vram.get_tile_bank_0_if_stale(), None);
        assert_eq!(vram.get_tile_bank_1_if_stale(), None);
        assert_eq!(vram.get_tile_bank_2_if_stale(), None);
        assert_eq!(vram.get_map_bank_0_if_stale(), None);
        assert_eq!(vram.get_map_bank_1_if_stale(), None);
    }

    #[test]
    fn has_address_in_bounds_is_true() {
        let vram = VRAM::new();

        assert!(vram.has_address(0x8000));
    }

    #[test]
    fn has_address_low_out_of_bounds_is_false() {
        let vram = VRAM::new();

        assert!(!vram.has_address(0x7FFF));
    }

    #[test]
    fn has_address_high_out_of_bounds_is_false() {
        let vram = VRAM::new();

        assert!(!vram.has_address(0xA000));
    }
}
