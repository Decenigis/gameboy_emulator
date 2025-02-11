use std::sync::{Arc};
use parking_lot::Mutex;
use crate::memory::io_map::io_trait::IOTrait;
use crate::memory::io_map::video_io::VideoIO;
use crate::memory::memory_trait::MemoryTrait;

pub struct IOMap {
    video_io: Arc<Mutex<VideoIO>>
}

impl MemoryTrait for IOMap {
    fn get(&self, position: u16) -> u8 {
        if self.video_io.lock().has_address(position) { self.video_io.lock().get(position) }
        else { 0xFF }
    }

    fn set(&mut self, position: u16, value: u8) -> u8 {
        if self.video_io.lock().has_address(position) { self.video_io.lock().set(position, value) }
        else { 0xFF }
    }
}

impl IOMap {
    pub fn new() -> Self {
        Self {
            video_io: Arc::new(Mutex::new(VideoIO::new()))
        }
    }

    pub fn get_video_io(&mut self) -> Arc<Mutex<VideoIO>> {
        self.video_io.clone()
    }
}