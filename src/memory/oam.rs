use crate::memory::MemoryTrait;
use crate::memory::object::Object;

pub struct OAM {
    data: Vec<Object>
}


impl MemoryTrait for OAM {
    fn get(&self, position: u16) -> u8 {
        self.data[((position % 0xA0) >> 2) as usize].get(position & 0x03)
    }

    fn set(&mut self, position: u16, value: u8) -> u8 {
        let old_value = self.data[((position % 0xA0) >> 2) as usize].get(position & 0x03);

        self.data[((position % 0xA0) >> 2) as usize].set(position & 0x03, value);

        old_value
    }

    fn has_address(&self, position: u16) -> bool {
        position >= 0xFE00 && position < 0xFEA0
    }
}

impl OAM {

    const OAM_SIZE: usize = 0xA0;

    pub fn new() -> Self {
        Self { data: vec![Object::new(); 40] }
    }
    
    pub fn get_objects(&self) -> &[Object] {
        self.data.as_slice()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_oam_stores_data() {
        let mut oam = OAM::new();

        oam.set(0xFE80, 0x12);

        assert_eq!(oam.get(0xFE80), 0x12);
    }

    #[test]
    fn has_address_in_bounds_is_true() {
        let oam = OAM::new();

        assert!(oam.has_address(0xFE90));
    }

    #[test]
    fn has_address_low_out_of_bounds_is_false() {
        let oam = OAM::new();

        assert!(!oam.has_address(0xFD7F));
    }

    #[test]
    fn has_address_high_out_of_bounds_is_false() {
        let oam = OAM::new();

        assert!(!oam.has_address(0xFEA0));
    }
}
