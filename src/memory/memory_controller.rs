use std::path::Path;
use std::sync::Arc;
use parking_lot::Mutex;
use crate::memory::hram::HRAM;
use crate::memory::io_map::IOMap;
use crate::memory::memory_trait::MemoryTrait;
use crate::memory::oam::OAM;
use crate::memory::ram::RAM;
use crate::memory::rom::ROM;
use crate::memory::sram::SRAM;
use crate::memory::vram::VRAM;

pub struct MemoryController {
    rom: ROM,
    vram:  Arc<Mutex<VRAM>>,
    sram: SRAM,
    ram: RAM,
    oam: OAM,
    io_map: Arc<Mutex<IOMap>>,
    hram: HRAM,
}

impl MemoryTrait for MemoryController {
    fn get(&self, position: u16) -> u8 {
        if self.rom.has_address(position) {
            self.rom.get(position)
        }
        else if self.vram.lock().has_address(position) {
            self.vram.lock().get(position)
        }
        else if self.ram.has_address(position) {
            self.ram.get(position)
        }
        else if self.io_map.lock().has_address(position) {
            self.io_map.lock().get(position)
        }
        else if self.hram.has_address(position) {
            self.hram.get(position)
        }
        else {
            0x00
        }
    }

    fn set(&mut self, position: u16, value: u8) -> u8 {
        if self.rom.has_address(position) {
            self.rom.set(position, value)
        }
        else if self.vram.lock().has_address(position) {
            self.vram.lock().set(position, value)
        }
        else if self.ram.has_address(position) {
            self.ram.set(position, value)
        }
        else if self.io_map.lock().has_address(position) {
            self.io_map.lock().set(position, value)
        }
        else if self.hram.has_address(position) {
            self.hram.set(position, value)
        }
        else {
            0xFF
        }
    }

    fn has_address(&self, _position: u16) -> bool {
        true
    }
}

impl MemoryController {
    pub fn new () -> Self {
        Self {
            rom: ROM::new(),
            vram: Arc::new(Mutex::new(VRAM::new())),
            sram: SRAM::new(),
            ram: RAM::new(),
            oam: OAM::new(),
            io_map: Arc::new(Mutex::new(IOMap::new())),
            hram: HRAM::new()
        }
    }

    pub fn load_rom(&mut self, path: &String) {
        self.rom.load_rom_file(Path::new(path));
    }

    pub fn get_vram_arc(&self) -> Arc<Mutex<VRAM>> {
        self.vram.clone()
    }

    pub fn get_io_map(&self) -> Arc<Mutex<IOMap>> {
        self.io_map.clone()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn writes_to_vram() {
        let expected_value = 0x12;
        let mut memory_controller = MemoryController::new();
        let vram = memory_controller.get_vram_arc();

        memory_controller.set(0x8000, expected_value);

        assert_eq!(vram.lock().get(0x8000), expected_value);
    }

    #[test]
    fn reads_from_vram() {
        let expected_value = 0x12;
        let memory_controller = MemoryController::new();
        let vram = memory_controller.get_vram_arc();

        vram.lock().set(0x8000, expected_value);

        assert_eq!(memory_controller.get(0x8000), expected_value);
    }


    #[test]
    fn writes_to_ram() {
        let expected_value = 0x12;
        let mut memory_controller = MemoryController::new();

        memory_controller.set(0xC000, expected_value);

        assert_eq!(memory_controller.ram.get(0xC000), expected_value);
    }

    #[test]
    fn reads_from_ram() {
        let expected_value = 0x12;
        let mut memory_controller = MemoryController::new();

        memory_controller.ram.set(0xC000, expected_value);

        assert_eq!(memory_controller.get(0xC000), expected_value);
    }


    #[test]
    fn writes_to_io_map() {
        let expected_value = 0x12;
        let mut memory_controller = MemoryController::new();

        memory_controller.set(0xFF40, expected_value);

        assert_eq!(memory_controller.io_map.lock().get(0xFF40), expected_value);
    }

    #[test]
    fn reads_from_io_map() {
        let expected_value = 0x12;
        let memory_controller = MemoryController::new();

        memory_controller.io_map.lock().set(0xFF40, expected_value);

        assert_eq!(memory_controller.get(0xFF40), expected_value);
    }

    #[test]
    fn writes_to_hram() {
        let expected_value = 0x12;
        let mut memory_controller = MemoryController::new();

        memory_controller.set(0xFF80, expected_value);

        assert_eq!(memory_controller.hram.get(0xFF80), expected_value);
    }

    #[test]
    fn reads_from_hram() {
        let expected_value = 0x12;
        let mut memory_controller = MemoryController::new();

        memory_controller.hram.set(0xFF80, expected_value);

        assert_eq!(memory_controller.get(0xFF80), expected_value);
    }

}
