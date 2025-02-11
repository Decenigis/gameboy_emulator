pub struct RAM { //also echo RAM
    data: Vec<Vec<u8>>,
}

const RAM_BANK_SIZE: usize = 0x1000;

impl RAM {
    pub fn new() -> Self {
        Self { data: vec![vec![0; RAM_BANK_SIZE], vec![0; RAM_BANK_SIZE]] }
    }
}