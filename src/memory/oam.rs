pub struct OAM {
    data: Vec<u8>
}

const OAM_SIZE: usize = 0xA0;

impl OAM {
    pub fn new() -> Self {
        Self {
            data: vec![0; OAM_SIZE]
        }
    }
}