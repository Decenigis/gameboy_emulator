pub struct HRAM {
    data: Vec<u8>,
}

const HRAM_SIZE: usize = 0x80;

impl HRAM {
    pub fn new() -> Self {
        Self { data: vec![0; HRAM_SIZE] }
    }
}