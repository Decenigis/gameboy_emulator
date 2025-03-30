#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Interrupt {
    VBlank,
    LCDStat,
    Timer,
    Serial,
    Joypad,
}

impl Interrupt {
    pub fn get_bit_mask(&self) -> u8 {
        match self {
            Interrupt::VBlank => 0b00000001,
            Interrupt::LCDStat => 0b00000010,
            Interrupt::Timer => 0b00000100,
            Interrupt::Serial => 0b00001000,
            Interrupt::Joypad => 0b00010000,
        }
    }
    
    pub fn get_address(&self) -> u16 {
        match self {
            Interrupt::VBlank => 0x0040,
            Interrupt::LCDStat => 0x0048,
            Interrupt::Timer => 0x0050,
            Interrupt::Serial => 0x0058,
            Interrupt::Joypad => 0x0060,
        }
    }
}
