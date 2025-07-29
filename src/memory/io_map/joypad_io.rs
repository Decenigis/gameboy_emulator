use crate::memory::MemoryTrait;

pub struct JoypadIO {
    up: bool,
    down: bool,
    left: bool,
    right: bool,

    a: bool,
    b: bool,
    select: bool,
    start: bool,

    memory_value: u8
}

impl MemoryTrait for JoypadIO {
    fn get(&self, position: u16) -> u8 {
        self.memory_value
    }

    fn set(&mut self, _position: u16, value: u8) -> u8 {
        let old_value = self.memory_value;

        self.memory_value = value | 0xC0;
        self.calculate_memory_value();

        old_value
    }

    fn has_address(&self, position: u16) -> bool {
        position == 0xFF00
    }
}

impl JoypadIO {

    pub fn new() -> Self{
        Self {
            up: false,
            down: false,
            left: false,
            right: false,

            a: false,
            b: false,
            select: false,
            start: false,

            memory_value: 0xCF
        }
    }

    pub fn calculate_memory_value(&mut self) {
        if self.memory_value & 0x30 == 0x20 {
            self.memory_value |= 0x0F;
            if self.right {
                self.memory_value &= 0b11111110;
            }
            if self.left {
                self.memory_value &= 0b11111101;
            }
            if self.up {
                self.memory_value &= 0b11111011;
            }
            if self.down {
                self.memory_value &= 0b11110111;
            }
        }
        else if self.memory_value & 0x30 == 0x10 {
            self.memory_value |= 0x0F;
            if self.a {
                self.memory_value &= 0b11111110;
            }
            if self.b {
                self.memory_value &= 0b11111101;
            }
            if self.select {
                self.memory_value &= 0b11111011;
            }
            if self.start {
                self.memory_value &= 0b11110111;
            }
        }
        else {
            self.memory_value |= 0xCF;
        }
    }

    pub fn set_right(&mut self, value: bool) {
        self.right = value;
        self.calculate_memory_value();
    }

    pub fn set_left(&mut self, value: bool) {
        self.left = value;
        self.calculate_memory_value();
    }

    pub fn set_up(&mut self, value: bool) {
        self.up = value;
        self.calculate_memory_value();
    }

    pub fn set_down(&mut self, value: bool) {
        self.down = value;
        self.calculate_memory_value();
    }

    pub fn set_a(&mut self, value: bool) {
        self.a = value;
        self.calculate_memory_value();
    }

    pub fn set_b(&mut self, value: bool) {
        self.b = value;
        self.calculate_memory_value();
    }

    pub fn set_select(&mut self, value: bool) {
        self.select = value;
        self.calculate_memory_value();
    }

    pub fn set_start(&mut self, value: bool) {
        self.start = value;
        self.calculate_memory_value();
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn responds_to_0xff00() {
        let joypad_io = JoypadIO::new();

        assert!(joypad_io.has_address(0xFF00));
    }

    #[test]
    fn does_not_respond_to_other_addresses() {
        let joypad_io = JoypadIO::new();

        assert!(!joypad_io.has_address(0xFFEF));
        assert!(!joypad_io.has_address(0xFF01));
    }

    #[test]
    fn unset_register_gives_0xcf() {
        let mut joypad_io = JoypadIO::new();
        joypad_io.set(0xFF00, 0x00);

        assert_eq!(0xCF, joypad_io.get(0xFF00));
    }

    #[test]
    fn invalid_register_set_gives_0xff() {
        let mut joypad_io = JoypadIO::new();
        joypad_io.set(0xFF00, 0x30);

        assert_eq!(0xFF, joypad_io.get(0xFF00));
    }

    #[test]
    fn gets_right_correctly() {
        let mut joypad_io = JoypadIO::new();

        joypad_io.set_right(true);

        joypad_io.set(0xFF00, 0x20);
        assert_eq!(0b11101110, joypad_io.get(0xFF00));
    }

    #[test]
    fn gets_left_correctly() {
        let mut joypad_io = JoypadIO::new();

        joypad_io.set_left(true);

        joypad_io.set(0xFF00, 0x20);
        assert_eq!(0b11101101, joypad_io.get(0xFF00));
    }

    #[test]
    fn gets_up_correctly() {
        let mut joypad_io = JoypadIO::new();

        joypad_io.set_up(true);

        joypad_io.set(0xFF00, 0x20);
        assert_eq!(0b11101011, joypad_io.get(0xFF00));
    }

    #[test]
    fn gets_down_correctly() {
        let mut joypad_io = JoypadIO::new();

        joypad_io.set_down(true);

        joypad_io.set(0xFF00, 0x20);
        assert_eq!(0b11100111, joypad_io.get(0xFF00));
    }

    #[test]
    fn gets_a_correctly() {
        let mut joypad_io = JoypadIO::new();

        joypad_io.set_a(true);

        joypad_io.set(0xFF00, 0x10);
        assert_eq!(0b11011110, joypad_io.get(0xFF00));
    }

    #[test]
    fn gets_b_correctly() {
        let mut joypad_io = JoypadIO::new();

        joypad_io.set_b(true);

        joypad_io.set(0xFF00, 0x10);
        assert_eq!(0b11011101, joypad_io.get(0xFF00));
    }

    #[test]
    fn gets_select_correctly() {
        let mut joypad_io = JoypadIO::new();

        joypad_io.set_select(true);

        joypad_io.set(0xFF00, 0x10);
        assert_eq!(0b11011011, joypad_io.get(0xFF00));
    }

    #[test]
    fn gets_start_correctly() {
        let mut joypad_io = JoypadIO::new();

        joypad_io.set_start(true);

        joypad_io.set(0xFF00, 0x10);
        assert_eq!(0b11010111, joypad_io.get(0xFF00));
    }
}
