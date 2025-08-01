use crate::memory::memory_trait::MemoryTrait;

pub struct VideoIO {
    lcd_ctrl: u8,   //0xFF40
    lcd_stat: u8,   //0xFF41
    scroll_y: u8,   //0xFF42
    scroll_x: u8,   //0xFF43
    ly: u8,         //0xFF44
    lyc: u8,        //0xFF45
    oam_dma: u8,    //0xFF46
    bg_pal: u8,     //0xFF47
    obj_pal_0: u8,  //0xFF48
    obj_pal_1: u8,  //0xFF49
    win_x: u8,      //0xFF4A
    win_y: u8,      //0xFF4B
}

impl MemoryTrait for VideoIO {
    fn get(&self, position: u16) -> u8 {
        match position {
            0xFF40 => self.lcd_ctrl,
            0xFF41 => self.lcd_stat,
            0xFF42 => self.scroll_y,
            0xFF43 => self.scroll_x,
            0xFF44 => self.ly,
            0xFF45 => self.lyc,
            0xFF46 => self.oam_dma,
            0xFF47 => self.bg_pal,
            0xFF48 => self.obj_pal_0,
            0xFF49 => self.obj_pal_1,
            0xFF4A => self.win_x,
            0xFF4B => self.win_y,
            _ => 0xFF
        }
    }

    fn set(&mut self, position: u16, value: u8) -> u8 {
        let reference = match position {
            0xFF40 => &mut self.lcd_ctrl,
            0xFF41 => &mut self.lcd_stat,
            0xFF42 => &mut self.scroll_y,
            0xFF43 => &mut self.scroll_x,
            0xFF44 => &mut self.ly,
            0xFF45 => &mut self.lyc,
            0xFF46 => &mut self.oam_dma,
            0xFF47 => &mut self.bg_pal,
            0xFF48 => &mut self.obj_pal_0,
            0xFF49 => &mut self.obj_pal_1,
            0xFF4A => &mut self.win_y,
            0xFF4B => &mut self.win_x,
            _ => {return 0xFF;}
        };

        let old_value = *reference;
        *reference = value;

        old_value
    }

    fn has_address(&self, address: u16) -> bool {
        address >= 0xFF40 && address <= 0xFF4B
    }
}


impl VideoIO {
    pub fn new() -> Self {
        Self {
            lcd_ctrl:   0x91,
            lcd_stat:   0x81,
            scroll_y:   0x00,
            scroll_x:   0x00,
            ly:         0x91,
            lyc:        0x00,
            oam_dma:    0xFF,
            bg_pal:     0xFC,
            obj_pal_0:  0xFF,
            obj_pal_1:  0xFF,
            win_x:      0x00,
            win_y:      0x00
        }
    }

    pub fn get_lcd_ctrl(&self) -> u8 {
        self.lcd_ctrl
    }

    pub fn get_lcd_stat(&self) -> u8 {
        self.lcd_stat
    }
    
    pub fn set_lcd_stat(&mut self, lcd_stat: u8)  {
        self.lcd_stat = lcd_stat;
    }

    pub fn get_bg_y(&self) -> u8 {
        self.scroll_y
    }

    pub fn get_bg_x(&self) -> u8 {
        self.scroll_x
    }

    pub fn get_ly(&self) -> u8 {
        self.ly
    }

    pub fn set_ly(&mut self, ly: u8) {
        self.ly = ly;
    }

    pub fn get_lyc(&self) -> u8 {
        self.lyc
    }

    pub fn _get_oam_dma(&self) -> u8 {
        self.oam_dma
    }

    pub fn get_bg_pal(&self) -> u8 {
        self.bg_pal
    }

    pub fn get_obj_pal_0(&self) -> u8 {
        self.obj_pal_0
    }

    pub fn get_obj_pal_1(&self) -> u8 {
        self.obj_pal_1
    }

    pub fn get_win_x(&self) -> u8 {
        self.win_x
    }

    pub fn get_win_y(&self) -> u8 {
        self.win_y
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn has_address_returns_true_for_all_video_io_addresses() {
        let video_io = VideoIO::new();

        assert_eq!(video_io.has_address(0xFF40), true);
        assert_eq!(video_io.has_address(0xFF41), true);
        assert_eq!(video_io.has_address(0xFF42), true);
        assert_eq!(video_io.has_address(0xFF43), true);
        assert_eq!(video_io.has_address(0xFF44), true);
        assert_eq!(video_io.has_address(0xFF45), true);
        assert_eq!(video_io.has_address(0xFF46), true);
        assert_eq!(video_io.has_address(0xFF47), true);
        assert_eq!(video_io.has_address(0xFF48), true);
        assert_eq!(video_io.has_address(0xFF49), true);
        assert_eq!(video_io.has_address(0xFF4A), true);
        assert_eq!(video_io.has_address(0xFF4B), true);
    }

    #[test]
    fn has_address_returns_false_for_non_video_io_addresses() {
        let video_io = VideoIO::new();

        assert_eq!(video_io.has_address(0xFF39), false);
        assert_eq!(video_io.has_address(0xFF4C), false);
    }
}
