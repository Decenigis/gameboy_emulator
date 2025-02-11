pub struct ROM {
    data: Vec<Vec<u8>>,
    active_bank: usize
}

const ROM_BANK_SIZE: usize = 0x4000;

impl ROM {
    pub fn new() -> Self {
        Self {
            data: vec![vec![0; ROM_BANK_SIZE], vec![0; ROM_BANK_SIZE]],
            active_bank: 1
        }
    }
}
