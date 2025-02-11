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
    io_map: IOMap,
    hram: HRAM,
}

impl MemoryTrait for MemoryController {
    fn get(&self, position: u16) -> u8 {
        if self.vram.lock().has_address(position) {
            self.vram.lock().get(position)
        }
        else if self.ram.has_address(position) {
            self.ram.get(position)
        }
        else if self.io_map.has_address(position) {
            self.io_map.get(position)
        }
        else {
            0xFF
        }
    }

    fn set(&mut self, position: u16, value: u8) -> u8 {
        if self.vram.lock().has_address(position) {
            self.vram.lock().set(position, value)
        }
        else if self.ram.has_address(position) {
            self.ram.set(position, value)
        }
        else if self.ram.has_address(position) {
            self.io_map.set(position, value)
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
            io_map: IOMap::new(),
            hram: HRAM::new()
        }
    }

    pub fn get_vram_arc(&self) -> Arc<Mutex<VRAM>> {
        self.vram.clone()
    }

    pub fn get_io_map(&self) -> &IOMap {
        &self.io_map
    }
}
