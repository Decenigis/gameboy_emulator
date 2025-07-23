#[derive(Clone)]
pub struct Object {
    x: u8,
    y: u8,
    tile: u8,
    attributes: u8,

    priority: bool,
    vertical_flip: bool,
    horizontal_flip: bool,
    dmg_palette: bool,
    cgb_bank: bool,
    cgb_palette: u8
}

impl Object {
    pub fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            tile: 0,
            attributes: 0,

            priority: false,
            vertical_flip: false,
            horizontal_flip: false,
            dmg_palette: false,
            cgb_bank: false,
            cgb_palette: 0
        }
    }

    pub fn set(&mut self, position: u16, value: u8) {
        match position {
            0 => self.x = value,
            1 => self.y = value,
            2 => self.tile = value,
            3 => {
                self.attributes = value;

                self.priority = (value & 0x80) != 0;
                self.vertical_flip = (value & 0x40) != 0;
                self.horizontal_flip = (value & 0x20) != 0;
                self.dmg_palette = (value & 0x10) != 0;
                self.cgb_bank = (value & 0x08) != 0;
                self.cgb_palette = value & 0x07;
            },
            _ => {}
        }
    }

    pub fn get(&self, position: u16) -> u8 {
        match position {
            0 => self.x,
            1 => self.y,
            2 => self.tile,
            3 => self.attributes,
            _ => 0xFF
        }
    }

    pub fn get_x(&self) -> u8 {
        self.x
    }

    pub fn get_y(&self) -> u8 {
        self.y
    }

    pub fn get_tile(&self) -> u8 {
        self.tile
    }

    pub fn get_priority(&self) -> bool {
        self.priority
    }

    pub fn get_vertical_flip(&self) -> bool {
        self.vertical_flip
    }

    pub fn get_horizontal_flip(&self) -> bool {
        self.horizontal_flip
    }

    pub fn get_dmg_palette(&self) -> bool {
        self.dmg_palette
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn x_is_set_properly() {
        let mut object = Object::new();
        object.set(0, 0x12);

        assert_eq!(object.get_x(), 0x12);
    }

    #[test]
    fn x_can_be_got_by_get() {
        let mut object = Object::new();
        object.set(0, 0x12);

        assert_eq!(object.get(0), 0x12);
    }

    #[test]
    fn y_is_set_properly() {
        let mut object = Object::new();
        object.set(1, 0x34);

        assert_eq!(object.get_y(), 0x34);
    }

    #[test]
    fn y_can_be_got_by_get() {
        let mut object = Object::new();
        object.set(1, 0x34);

        assert_eq!(object.get(1), 0x34);
    }

    #[test]
    fn tile_is_set_properly() {
        let mut object = Object::new();
        object.set(2, 0x56);

        assert_eq!(object.get_tile(), 0x56);
    }

    #[test]
    fn tile_can_be_got_by_get() {
        let mut object = Object::new();
        object.set(2, 0x56);

        assert_eq!(object.get(2), 0x56);
    }

    #[test]
    fn priority_is_set_properly() {
        let mut object = Object::new();
        object.set(3, 0b10000000);

        assert!(object.get_priority());
    }

    #[test]
    fn vertical_flip_is_set_properly() {
        let mut object = Object::new();
        object.set(3, 0b01000000);

        assert!(object.get_vertical_flip());
    }

    #[test]
    fn horizontal_flip_is_set_properly() {
        let mut object = Object::new();
        object.set(3, 0b00100000);

        assert!(object.get_horizontal_flip());
    }

    #[test]
    fn dmg_palette_is_set_properly() {
        let mut object = Object::new();
        object.set(3, 0b00010000);

        assert!(object.get_dmg_palette());
    }

    #[test]
    fn cgb_bank_is_set_properly() {
        let mut object = Object::new();
        object.set(3, 0b00001000);

        assert!(object.cgb_bank);
    }

    #[test]
    fn cgb_palette_is_set_properly() {
        let mut object = Object::new();
        object.set(3, 0b00000111);

        assert_eq!(object.cgb_palette, 0x07);
    }

    #[test]
    fn attributes_can_be_got_by_get() {
        let mut object = Object::new();
        object.set(3, 0x12); // Set some attributes

        assert_eq!(object.get(3), 0x12);
    }
}
