use std::sync::{Arc};
use parking_lot::Mutex;
use crate::memory::io_map::interrupt_io::InterruptIO;
use crate::memory::io_map::JoypadIO;
use crate::memory::io_map::video_io::VideoIO;
use crate::memory::memory_trait::MemoryTrait;

pub struct IOMap {
    joypad_io: Arc<Mutex<JoypadIO>>,
    interrupt_io: InterruptIO,
    video_io: Arc<Mutex<VideoIO>>,
}

impl MemoryTrait for IOMap {
    fn get(&self, position: u16) -> u8 {
        if self.joypad_io.lock().has_address(position) { self.joypad_io.lock().get(position) }
        else if self.interrupt_io.has_address(position) { self.interrupt_io.get(position) }
        else if self.video_io.lock().has_address(position) { self.video_io.lock().get(position) }
        else { 0xFF }
    }

    fn set(&mut self, position: u16, value: u8) -> u8 {
        if self.joypad_io.lock().has_address(position) { self.joypad_io.lock().set(position, value) }
        else if self.interrupt_io.has_address(position) { self.interrupt_io.set(position, value) }
        else if self.video_io.lock().has_address(position) { self.video_io.lock().set(position, value) }
        else { 0xFF }
    }

    fn has_address(&self, position: u16) -> bool { //all members must be or'd together for this
        self.joypad_io.lock().has_address(position) ||
        self.interrupt_io.has_address(position) ||
        self.video_io.lock().has_address(position)
    }
}

impl IOMap {
    pub fn new() -> Self {
        Self {
            joypad_io: Arc::new(Mutex::new(JoypadIO::new())),
            interrupt_io: InterruptIO::new(),
            video_io: Arc::new(Mutex::new(VideoIO::new()))
        }
    }
    
    pub fn reset(&mut self) {
        *self.joypad_io.lock() = JoypadIO::new();
        self.interrupt_io = InterruptIO::new();
        *self.video_io.lock() = VideoIO::new();
    }

    pub fn get_joypad_io(&self) -> Arc<Mutex<JoypadIO>> {
        self.joypad_io.clone()
    }

    pub fn get_video_io(&self) -> Arc<Mutex<VideoIO>> {
        self.video_io.clone()
    }
}
