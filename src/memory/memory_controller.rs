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
    oam_dma_position: u16,
    oam_dma_address: u16,
    performing_dma: bool,

    rom: ROM,
    vram:  Arc<Mutex<VRAM>>,
    sram: SRAM,
    ram: RAM,
    oam: Arc<Mutex<OAM>>,
    io_map: Arc<Mutex<IOMap>>,
    hram: HRAM,
}

impl MemoryTrait for MemoryController {
    fn get(&self, position: u16) -> u8 {
        if !self.performing_dma && position < 0xFF00 && self.oam_dma_position < 160 {
            return 0xFF;
        }

        if self.rom.has_address(position) {
            self.rom.get(position)
        }
        else if self.vram.lock().has_address(position) {
            self.vram.lock().get(position)
        }
        else if self.ram.has_address(position) {
            self.ram.get(position)
        }
        else if self.oam.lock().has_address(position) {
            self.oam.lock().get(position)
        }
        else if self.io_map.lock().has_address(position) {
            self.io_map.lock().get(position)
        }
        else if self.hram.has_address(position) {
            self.hram.get(position)
        }
        else {
            0xFF
        }
    }

    fn set(&mut self, position: u16, value: u8) -> u8 {
        if !self.performing_dma && position < 0xFF00 && self.oam_dma_position < 160 {
            return 0xFF;
        }
        if position == 0xFF46 {
            self.oam_dma_position = 0;
            self.oam_dma_address = (value as u16) << 8;
        }

        if self.rom.has_address(position) {
            self.rom.set(position, value)
        }
        else if self.vram.lock().has_address(position) {
            self.vram.lock().set(position, value)
        }
        else if self.ram.has_address(position) {
            self.ram.set(position, value)
        }
        else if self.oam.lock().has_address(position) {
            self.oam.lock().set(position, value)
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
            oam_dma_address: 0,
            oam_dma_position: 160,
            performing_dma: false,

            rom: ROM::new(),
            vram: Arc::new(Mutex::new(VRAM::new())),
            sram: SRAM::new(),
            ram: RAM::new(),
            oam: Arc::new(Mutex::new(OAM::new())),
            io_map: Arc::new(Mutex::new(IOMap::new())),
            hram: HRAM::new()
        }
    }

    pub fn clock(&mut self) {
        if self.oam_dma_position < 160 {
            self.performing_dma = true;

            for _ in 0..4 { //because the emulator only emulates every 4 clock cycles
                let value = self.get(self.oam_dma_address | self.oam_dma_position);
                self.set(0xFE00 + self.oam_dma_position, value);

                self.oam_dma_position += 1;
            }

            self.performing_dma = false;
        }
    }

    pub fn load_rom(&mut self, path: &String) {
        self.rom.load_rom_file(Path::new(path));
    }

    pub fn get_vram_arc(&self) -> Arc<Mutex<VRAM>> {
        self.vram.clone()
    }

    pub fn get_oam_arc(&self) -> Arc<Mutex<OAM>> {
        self.oam.clone()
    }

    pub fn get_io_map(&self) -> Arc<Mutex<IOMap>> {
        self.io_map.clone()
    }

    pub fn get_rom(&self) -> &ROM {
        &self.rom
    }
    
    pub fn reset(&mut self) {
        self.oam_dma_position = 160;
        self.oam_dma_address = 0;
        self.performing_dma = false;

        self.rom = ROM::new();
        *self.vram.lock() = VRAM::new();
        self.sram = SRAM::new();
        self.ram = RAM::new();
        
        *self.oam.lock() = OAM::new();
        self.io_map.lock().reset();
        
        self.hram = HRAM::new();
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
    fn writes_to_oam() {
        let expected_value = 0x12;
        let mut memory_controller = MemoryController::new();

        memory_controller.set(0xFE00, expected_value);

        assert_eq!(memory_controller.oam.lock().get(0xFE00), expected_value);
    }

    #[test]
    fn reads_from_oam() {
        let expected_value = 0x12;
        let mut memory_controller = MemoryController::new();

        memory_controller.oam.lock().set(0xFE00, expected_value);

        assert_eq!(memory_controller.get(0xFE00), expected_value);
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

    #[test]
    fn copies_oam_during_dma() {
        let mut memory_controller = MemoryController::new();

        for i in 0..0xA0 {
            memory_controller.set(0xD000 | i, i as u8);
        }

        memory_controller.set(0xFF46, 0xD0);

        for i in 0..40 {
            memory_controller.clock();
        }

        for i in 0..0xA0 {
            assert_eq!(memory_controller.get(0xFE00 + i), i as u8);
        }
    }

    #[test]
    fn locks_memory_during_dma() {
        let mut memory_controller = MemoryController::new();

        memory_controller.set(0xFF46, 0xD0);

        assert_eq!(0xFF, memory_controller.get(0xD000));
        assert_eq!(0xFF, memory_controller.get(0xFE00));
    }
}
