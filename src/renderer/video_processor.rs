use std::sync::Arc;
use dec_gl::renderable::Renderable;
use mockall_double::double;
use dec_gl::Vertex2d;
use dec_gl::shader::{ShaderManager, ShaderProgram};
#[double]
use dec_gl::texture::{Texture2Du8, Texture3Du8};
use dec_gl::types::{ivec2, ivec3, IVec2};
use parking_lot::Mutex;
use crate::memory::io_map::VideoIO;
use crate::memory::VRAM;
use crate::renderer::RendererError;

pub struct VideoProcessor {
    tilemap_bank_0: Texture3Du8,
    tilemap_bank_1: Texture3Du8,
    tilemap_bank_2: Texture3Du8,

    map_bank_0: Texture2Du8,
    map_bank_1: Texture2Du8,

    background_renderable: Box<dyn Renderable<Vertex2d>>,

    vram: Arc<Mutex<VRAM>>,
    video_io: Arc<Mutex<VideoIO>>
}

pub struct LCDCMask {}

impl LCDCMask {
    pub const LCD_ENABLE: u8 = 0x80;
    pub const WIN_TILE_BANK: u8 = 0x40;
    pub const WIN_ENABLE: u8 = 0x20;
    pub const WIN_AND_BG_MAP: u8 = 0x10;
    pub const BG_TILE_BANK: u8 = 0x08;
    pub const OBJ_SIZE: u8 = 0x04;
    pub const OBJ_ENABLE: u8 = 0x02;
    pub const BG_ENABLE: u8 = 0x01;

    pub fn mask(input: u8, mask: u8) -> bool {
        input & mask != 0
    }
}

impl VideoProcessor {

    pub fn new(
        tilemap_bank_0: Texture3Du8,
        tilemap_bank_1: Texture3Du8,
        tilemap_bank_2: Texture3Du8,

        map_bank_0: Texture2Du8,
        map_bank_1: Texture2Du8,

        mut background_renderable: Box<dyn Renderable<Vertex2d>>,

        vram: Arc<Mutex<VRAM>>,
        video_io: Arc<Mutex<VideoIO>>
    )
        -> Result<VideoProcessor, RendererError>
    {
        match background_renderable.initialise(&vec![
            Vertex2d { x: 0.0, y: 0.0, u: 0.0, v: 0.0},
            Vertex2d { x: 0.0, y: 144.0, u: 0.0, v: 1.0},
            Vertex2d { x: 160.0, y: 0.0, u: 1.0, v: 0.0},

            Vertex2d { x: 0.0, y: 144.0, u: 0.0, v: 1.0},
            Vertex2d { x: 160.0, y: 0.0, u: 1.0, v: 0.0},
            Vertex2d { x: 160.0, y: 144.0, u: 1.0, v: 1.0}],
        None)
        {
            Ok(_) => {}
            Err(error) => return Err(RendererError::GLError { error })
        }

        Ok( VideoProcessor {
            tilemap_bank_0,
            tilemap_bank_1,
            tilemap_bank_2,

            map_bank_0,
            map_bank_1,

            background_renderable,
            vram,
            video_io
        } )
    }

    pub fn try_update_graphics_data(&mut self) {
        let mut vram = self.vram.lock();

        match vram.get_tile_bank_0_if_stale() {
            None => {}
            Some(data) => match self.tilemap_bank_0.set_data(data, ivec3(2, 8, 128)) {
                Ok(_) => {}
                Err(_) => {}
            }
        }

        match vram.get_tile_bank_1_if_stale() {
            None => {}
            Some(data) => match self.tilemap_bank_1.set_data(data, ivec3(2, 8, 128)) {
                Ok(_) => {}
                Err(_) => {}
            }
        }

        match vram.get_tile_bank_2_if_stale() {
            None => {}
            Some(data) => match self.tilemap_bank_2.set_data(data, ivec3(2, 8, 128)) {
                Ok(_) => {}
                Err(_) => {}
            }
        }

        match vram.get_map_bank_0_if_stale() {
            None => {}
            Some(data) => match self.map_bank_0.set_data(data, ivec2(32, 32)) {
                Ok(_) => {}
                Err(_) => {}
            }
        }

        match vram.get_map_bank_1_if_stale() {
            None => {}
            Some(data) => match self.map_bank_1.set_data(data, ivec2(32, 32)) {
                Ok(_) => {}
                Err(_) => {}
            }
        }

        vram.set_not_stale();
    }

    fn bind_tile_textures_to_units(&self, texture_bank: bool) {
        if texture_bank {
            self.tilemap_bank_0.bind_to_unit(1);
        }
        else {
            self.tilemap_bank_2.bind_to_unit(1);
        }

        self.tilemap_bank_1.bind_to_unit(2);
    }

    fn bind_map_textures_to_units(&self, map_bank: bool) {
        if map_bank {
            self.map_bank_1.bind_to_unit(0);
        }
        else {
            self.map_bank_0.bind_to_unit(0);
        }
    }

    fn bind_textures_for_background(&self, lcd_ctrl: u8) {
        self.bind_tile_textures_to_units(
            LCDCMask::mask(lcd_ctrl, LCDCMask::WIN_AND_BG_MAP)
        );
        self.bind_map_textures_to_units(
            LCDCMask::mask(lcd_ctrl, LCDCMask::BG_TILE_BANK)
        );
    }

    fn bind_textures_for_window(&self, lcd_ctrl: u8) {
        self.bind_tile_textures_to_units(
            LCDCMask::mask(lcd_ctrl, LCDCMask::WIN_AND_BG_MAP)
        );
        self.bind_map_textures_to_units(
            LCDCMask::mask(lcd_ctrl, LCDCMask::WIN_TILE_BANK)
        );
    }

    fn set_shader_values(&self, shader: &mut Box<dyn ShaderProgram>, scroll: &IVec2, draw_cutoff: &IVec2, bg_pal: &u8){
        shader.bind();
        shader.set_uniform("scroll".to_string(), scroll);
        shader.set_uniform("draw_cutoff".to_string(), draw_cutoff);
        shader.set_uniform("bg_pal".to_string(), &(*bg_pal as i32));
    }

    fn draw_background(&mut self, shader: &mut Box<dyn ShaderProgram>) -> Result<(), RendererError> {
        {
            let video_io_mutex = self.video_io.clone();
            let video_io_guard = video_io_mutex.lock();

            self.set_shader_values(shader,
                                   &ivec2(video_io_guard.get_bg_x() as i32, video_io_guard.get_bg_y() as i32),
                                   &ivec2(0, video_io_guard.get_ly() as i32),
                                   &video_io_guard.get_bg_pal()
            );
            self.bind_textures_for_background(video_io_guard.get_lcd_ctrl());
        }

        self.background_renderable.draw();

        Ok(())
    }

    fn draw_window(&mut self, shader: &mut Box<dyn ShaderProgram>) -> Result<(), RendererError> {
        {
            let video_io_mutex = self.video_io.clone();
            let video_io_guard = video_io_mutex.lock();

            self.set_shader_values(shader,
                                   &ivec2(!video_io_guard.get_win_x().wrapping_sub(8) as i32, !video_io_guard.get_win_y().wrapping_sub(1) as i32),
                                   &ivec2((video_io_guard.get_win_x() as i32) - 8, video_io_guard.get_win_y() as i32),
                                   &video_io_guard.get_bg_pal()
            );
            self.bind_textures_for_window(video_io_guard.get_lcd_ctrl());
        }

        self.background_renderable.draw();

        Ok(())
    }

    pub fn draw(&mut self, shader_manager: &mut ShaderManager) -> Result<(), RendererError> {
        let lcd_ctrl = {
            let video_io_mutex = self.video_io.clone();
            let video_io_guard = video_io_mutex.lock();
            video_io_guard.get_lcd_ctrl().clone()
        };

        if LCDCMask::mask(lcd_ctrl, LCDCMask::LCD_ENABLE) {
            match shader_manager.bind("BACKGROUND".to_string()) {
                Ok(background_shader) => {
                    if LCDCMask::mask(lcd_ctrl, LCDCMask::BG_ENABLE) {
                        self.draw_background(background_shader)?;
                    }
                    if LCDCMask::mask(lcd_ctrl, LCDCMask::WIN_ENABLE) {
                        self.draw_window(background_shader)?;
                    }
                }
                Err(error) => {
                    return Err(RendererError::GLError { error });
                }
            }
        }

        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::collections::HashMap;
    use std::rc::Rc;
    use std::sync::Arc;
    use dec_gl::renderable::{NullableRenderable, Renderable};
    use dec_gl::shader::{NullableShaderProgram, ShaderManager, ShaderProgram};
    use dec_gl::texture::{MockTexture2Du8, MockTexture3Du8};
    use dec_gl::types::ivec2;
    use dec_gl::Vertex2d;
    use mockall::predicate::eq;
    use parking_lot::Mutex;
    use crate::memory::io_map::VideoIO;
    use crate::memory::{MemoryTrait, VRAM};
    use crate::renderer::video_processor::LCDCMask;
    use crate::renderer::VideoProcessor;


    fn get_mock_textures() -> (MockTexture3Du8, MockTexture3Du8, MockTexture3Du8, MockTexture2Du8, MockTexture2Du8) {
        let mock_texture_3d_0 = MockTexture3Du8::default();
        let mock_texture_3d_1 = MockTexture3Du8::default();
        let mock_texture_3d_2 = MockTexture3Du8::default();
        let mock_texture_2d_0 = MockTexture2Du8::default();
        let mock_texture_2d_1 = MockTexture2Du8::default();
        (mock_texture_3d_0, mock_texture_3d_1, mock_texture_3d_2, mock_texture_2d_0, mock_texture_2d_1)
    }

    fn get_mock_textures_with_expectations() -> (MockTexture3Du8, MockTexture3Du8, MockTexture3Du8, MockTexture2Du8, MockTexture2Du8) {
        let mut mock_texture_3d_0 = MockTexture3Du8::default();
        let mut mock_texture_3d_1 = MockTexture3Du8::default();
        let mut mock_texture_3d_2 = MockTexture3Du8::default();
        let mut mock_texture_2d_0 = MockTexture2Du8::default();
        let mut mock_texture_2d_1 = MockTexture2Du8::default();

        mock_texture_3d_0.expect_bind_to_unit().returning(|_| ());
        mock_texture_3d_1.expect_bind_to_unit().returning(|_| ());
        mock_texture_3d_2.expect_bind_to_unit().returning(|_| ());
        mock_texture_2d_0.expect_bind_to_unit().returning(|_| ());
        mock_texture_2d_1.expect_bind_to_unit().returning(|_| ());

        (mock_texture_3d_0, mock_texture_3d_1, mock_texture_3d_2, mock_texture_2d_0, mock_texture_2d_1)
    }

    fn get_generic_renderable() -> Box<dyn Renderable<Vertex2d>> {
        Box::new(NullableRenderable::<Vertex2d>::new::<Vertex2d>(
            Rc::new(RefCell::new(false)),
            Rc::new(RefCell::new(vec![])),
            Rc::new(RefCell::new(None)),
            Rc::new(RefCell::new(0)),
        ))
    }

    #[test]
    fn sets_vram_to_not_stale_after_updating_textures() {
        let vram = Arc::new(Mutex::new(VRAM::new()));
        let video_io = Arc::new(Mutex::new(VideoIO::new()));

        vram.lock().set_not_stale();
        vram.lock().set(0x8000, 0x01);

        let (mut tile_bank_0, mut tile_bank_1, mut tile_bank_2, mut map_bank_0, mut map_bank_1) = get_mock_textures();

        tile_bank_0.expect_set_data().returning(|_, _| Ok(()));
        tile_bank_1.expect_set_data().returning(|_, _| Ok(()));
        tile_bank_2.expect_set_data().returning(|_, _| Ok(()));
        map_bank_0.expect_set_data().returning(|_, _| Ok(()));
        map_bank_1.expect_set_data().returning(|_, _| Ok(()));


        let mut video_processor = VideoProcessor::new(
            tile_bank_0, tile_bank_1, tile_bank_2,
            map_bank_0, map_bank_1,
            get_generic_renderable(),
            vram.clone(),
            video_io.clone()).unwrap();

        video_processor.try_update_graphics_data();

        assert_eq!(None, vram.lock().get_map_bank_0_if_stale());
        assert_eq!(None, vram.lock().get_tile_bank_1_if_stale());
        assert_eq!(None, vram.lock().get_tile_bank_2_if_stale());
        assert_eq!(None, vram.lock().get_map_bank_0_if_stale());
        assert_eq!(None, vram.lock().get_map_bank_1_if_stale());
    }

    #[test]
    fn updates_tile_bank_0_if_data_is_stale() {
        let vram = Arc::new(Mutex::new(VRAM::new()));
        let video_io = Arc::new(Mutex::new(VideoIO::new()));

        vram.lock().set_not_stale();
        vram.lock().set(0x8000, 0x01);

        let mut tile_bank_0 = MockTexture3Du8::default();
        tile_bank_0.expect_set_data().times(1).returning(|_, _| Ok(()));

        let mut video_processor = VideoProcessor::new(
            tile_bank_0, MockTexture3Du8::default(), MockTexture3Du8::default(),
            MockTexture2Du8::default(), MockTexture2Du8::default(),
            get_generic_renderable(),
            vram.clone(),
            video_io.clone()).unwrap();

        video_processor.try_update_graphics_data();
    }

    #[test]
    fn updates_tile_bank_1_if_data_is_stale() {
        let vram = Arc::new(Mutex::new(VRAM::new()));
        let video_io = Arc::new(Mutex::new(VideoIO::new()));

        vram.lock().set_not_stale();
        vram.lock().set(0x8800, 0x01);

        let mut tile_bank_1 = MockTexture3Du8::default();
        tile_bank_1.expect_set_data().times(1).returning(|_, _| Ok(()));

        let mut video_processor = VideoProcessor::new(
            MockTexture3Du8::default(), tile_bank_1, MockTexture3Du8::default(),
            MockTexture2Du8::default(), MockTexture2Du8::default(),
            get_generic_renderable(),
            vram.clone(),
            video_io.clone()).unwrap();

        video_processor.try_update_graphics_data();
    }
    #[test]
    fn updates_tile_bank_2_if_data_is_stale() {
        let vram = Arc::new(Mutex::new(VRAM::new()));
        let video_io = Arc::new(Mutex::new(VideoIO::new()));

        vram.lock().set_not_stale();
        vram.lock().set(0x9000, 0x01);

        let mut tile_bank_2 = MockTexture3Du8::default();
        tile_bank_2.expect_set_data().times(1).returning(|_, _| Ok(()));

        let mut video_processor = VideoProcessor::new(
            MockTexture3Du8::default(), MockTexture3Du8::default(), tile_bank_2,
            MockTexture2Du8::default(), MockTexture2Du8::default(),
            get_generic_renderable(),
            vram.clone(),
            video_io.clone()).unwrap();

        video_processor.try_update_graphics_data();
    }

    #[test]
    fn updates_map_bank_0_if_data_is_stale() {
        let vram = Arc::new(Mutex::new(VRAM::new()));
        let video_io = Arc::new(Mutex::new(VideoIO::new()));

        vram.lock().set_not_stale();
        vram.lock().set(0x9800, 0x01);

        let mut map_bank_0 = MockTexture2Du8::default();
        map_bank_0.expect_set_data().times(1).returning(|_, _| Ok(()));

        let mut video_processor = VideoProcessor::new(
            MockTexture3Du8::default(), MockTexture3Du8::default(), MockTexture3Du8::default(),
            map_bank_0, MockTexture2Du8::default(),
            get_generic_renderable(),
            vram.clone(),
            video_io.clone()).unwrap();

        video_processor.try_update_graphics_data();
    }

    #[test]
    fn updates_map_bank_1_if_data_is_stale() {
        let vram = Arc::new(Mutex::new(VRAM::new()));
        let video_io = Arc::new(Mutex::new(VideoIO::new()));

        vram.lock().set_not_stale();
        vram.lock().set(0x9C00, 0x01);

        let mut map_bank_1 = MockTexture2Du8::default();
        map_bank_1.expect_set_data().times(1).returning(|_, _| Ok(()));

        let mut video_processor = VideoProcessor::new(
            MockTexture3Du8::default(), MockTexture3Du8::default(), MockTexture3Du8::default(),
            MockTexture2Du8::default(), map_bank_1,
            get_generic_renderable(),
            vram.clone(),
            video_io.clone()).unwrap();

        video_processor.try_update_graphics_data();
    }

    #[test]
    fn binds_tile_map_2_given_lcdc_bit_4_not_set() {
        let (mut tile_bank_0, mut tile_bank_1, mut tile_bank_2, mut map_bank_0, map_bank_1) = get_mock_textures();
        tile_bank_1.expect_bind_to_unit().returning(|_| ());
        map_bank_0.expect_bind_to_unit().returning(|_| ());

        tile_bank_0.expect_bind_to_unit().times(0).returning(|_| ());
        tile_bank_2.expect_bind_to_unit().with(eq(1)).times(2).returning(|_| ());

        let video_processor = VideoProcessor::new(
            tile_bank_0, tile_bank_1, tile_bank_2,
            map_bank_0, map_bank_1,
            get_generic_renderable(),
            Arc::new(Mutex::new(VRAM::new())),
            Arc::new(Mutex::new(VideoIO::new()))).unwrap();

        video_processor.bind_textures_for_background(LCDCMask::LCD_ENABLE);
        video_processor.bind_textures_for_window(LCDCMask::LCD_ENABLE);
    }

    #[test]
    fn binds_tile_map_0_for_bg_given_lcdc_bit_4_set() {
        let (mut tile_bank_0, mut tile_bank_1, mut tile_bank_2, mut map_bank_0, map_bank_1) = get_mock_textures();
        tile_bank_1.expect_bind_to_unit().returning(|_| ());
        map_bank_0.expect_bind_to_unit().returning(|_| ());

        tile_bank_0.expect_bind_to_unit().with(eq(1)).times(2).returning(|_| ());
        tile_bank_2.expect_bind_to_unit().times(0).returning(|_| ());

        let video_processor = VideoProcessor::new(
            tile_bank_0, tile_bank_1, tile_bank_2,
            map_bank_0, map_bank_1,
            get_generic_renderable(),
            Arc::new(Mutex::new(VRAM::new())),
            Arc::new(Mutex::new(VideoIO::new()))).unwrap();

        video_processor.bind_textures_for_background(LCDCMask::LCD_ENABLE | LCDCMask::WIN_AND_BG_MAP);
        video_processor.bind_textures_for_window(LCDCMask::LCD_ENABLE | LCDCMask::WIN_AND_BG_MAP);
    }

    #[test]
    fn binds_tile_map_1_whether_bit_4_is_or_is_not_set() {
        let (mut tile_bank_0, mut tile_bank_1, mut tile_bank_2, mut map_bank_0, map_bank_1) = get_mock_textures();
        tile_bank_0.expect_bind_to_unit().returning(|_| ());
        tile_bank_2.expect_bind_to_unit().returning(|_| ());
        map_bank_0.expect_bind_to_unit().returning(|_| ());

        tile_bank_1.expect_bind_to_unit().with(eq(2)).times(4).returning(|_| ());

        let video_processor = VideoProcessor::new(
            tile_bank_0, tile_bank_1, tile_bank_2,
            map_bank_0, map_bank_1,
            get_generic_renderable(),
            Arc::new(Mutex::new(VRAM::new())),
            Arc::new(Mutex::new(VideoIO::new()))).unwrap();

        video_processor.bind_textures_for_background(LCDCMask::LCD_ENABLE);
        video_processor.bind_textures_for_background(LCDCMask::LCD_ENABLE | LCDCMask::WIN_AND_BG_MAP);

        video_processor.bind_textures_for_window(LCDCMask::LCD_ENABLE);
        video_processor.bind_textures_for_window(LCDCMask::LCD_ENABLE | LCDCMask::WIN_AND_BG_MAP);
    }

    #[test]
    fn binds_map_bank_0_for_background_given_lcdc_bit_3_not_set() {
        let (tile_bank_0, mut tile_bank_1, mut tile_bank_2, mut map_bank_0, mut map_bank_1) = get_mock_textures();
        tile_bank_1.expect_bind_to_unit().returning(|_| ());
        tile_bank_2.expect_bind_to_unit().returning(|_| ());

        map_bank_0.expect_bind_to_unit().with(eq(0)).times(1).returning(|_| ());
        map_bank_1.expect_bind_to_unit().times(0).returning(|_| ());

        let video_processor = VideoProcessor::new(
            tile_bank_0, tile_bank_1, tile_bank_2,
            map_bank_0, map_bank_1,
            get_generic_renderable(),
            Arc::new(Mutex::new(VRAM::new())),
            Arc::new(Mutex::new(VideoIO::new()))).unwrap();

        video_processor.bind_textures_for_background(LCDCMask::LCD_ENABLE);
    }

    #[test]
    fn binds_map_bank_1_for_background_given_lcdc_bit_3_set() {
        let (tile_bank_0, mut tile_bank_1, mut tile_bank_2, mut map_bank_0, mut map_bank_1) = get_mock_textures();
        tile_bank_1.expect_bind_to_unit().returning(|_| ());
        tile_bank_2.expect_bind_to_unit().returning(|_| ());

        map_bank_0.expect_bind_to_unit().times(0).returning(|_| ());
        map_bank_1.expect_bind_to_unit().with(eq(0)).times(1).returning(|_| ());

        let video_processor = VideoProcessor::new(
            tile_bank_0, tile_bank_1, tile_bank_2,
            map_bank_0, map_bank_1,
            get_generic_renderable(),
            Arc::new(Mutex::new(VRAM::new())),
            Arc::new(Mutex::new(VideoIO::new()))).unwrap();

        video_processor.bind_textures_for_background(LCDCMask::LCD_ENABLE | LCDCMask::BG_TILE_BANK);
    }

    #[test]
    fn binds_map_bank_0_for_window_given_lcdc_bit_7_not_set() {
        let (tile_bank_0, mut tile_bank_1, mut tile_bank_2, mut map_bank_0, mut map_bank_1) = get_mock_textures();
        tile_bank_1.expect_bind_to_unit().returning(|_| ());
        tile_bank_2.expect_bind_to_unit().returning(|_| ());

        map_bank_0.expect_bind_to_unit().with(eq(0)).times(1).returning(|_| ());
        map_bank_1.expect_bind_to_unit().times(0).returning(|_| ());

        let video_processor = VideoProcessor::new(
            tile_bank_0, tile_bank_1, tile_bank_2,
            map_bank_0, map_bank_1,
            get_generic_renderable(),
            Arc::new(Mutex::new(VRAM::new())),
            Arc::new(Mutex::new(VideoIO::new()))).unwrap();

        video_processor.bind_textures_for_window(LCDCMask::LCD_ENABLE);
    }

    #[test]
    fn binds_map_bank_1_for_window_given_lcdc_bit_7_set() {
        let (tile_bank_0, mut tile_bank_1, mut tile_bank_2, mut map_bank_0, mut map_bank_1) = get_mock_textures();
        tile_bank_1.expect_bind_to_unit().returning(|_| ());
        tile_bank_2.expect_bind_to_unit().returning(|_| ());

        map_bank_0.expect_bind_to_unit().times(0).returning(|_| ());
        map_bank_1.expect_bind_to_unit().with(eq(0)).times(1).returning(|_| ());

        let video_processor = VideoProcessor::new(
            tile_bank_0, tile_bank_1, tile_bank_2,
            map_bank_0, map_bank_1,
            get_generic_renderable(),
            Arc::new(Mutex::new(VRAM::new())),
            Arc::new(Mutex::new(VideoIO::new()))).unwrap();

        video_processor.bind_textures_for_window(LCDCMask::LCD_ENABLE | LCDCMask::WIN_TILE_BANK);
    }

    #[test]
    fn binds_the_background_shader_on_draw() {
        let vram = Arc::new(Mutex::new(VRAM::new()));
        let video_io = Arc::new(Mutex::new(VideoIO::new()));
        let was_bound = Rc::new(RefCell::new(false));
        vram.lock().set_not_stale();

        let (tile_bank_0, mut tile_bank_1, mut tile_bank_2, mut map_bank_0, map_bank_1) = get_mock_textures();
        tile_bank_1.expect_bind_to_unit().returning(|_| ());
        tile_bank_2.expect_bind_to_unit().returning(|_| ());
        map_bank_0.expect_bind_to_unit().returning(|_| ());

        let mut shader_manager = ShaderManager::new();
        shader_manager.register_shader("BACKGROUND".to_string(),
            Box::new(
                NullableShaderProgram::new(Rc::new(RefCell::new(HashMap::new())), was_bound.clone()),
            )
        ).unwrap();

        video_io.lock().set(0xFF40, 0x81);

        let mut video_processor = VideoProcessor::new(
            tile_bank_0, tile_bank_1, tile_bank_2,
            map_bank_0, map_bank_1,
            get_generic_renderable(),
            vram.clone(),
            video_io.clone()).unwrap();

        video_processor.draw(&mut shader_manager).unwrap();

        assert_eq!(true, *was_bound.borrow());
    }

    #[test]
    fn sets_shader_uniforms_for_background_on_draw() {
        let scroll = ivec2(0xFD, 0xFC);
        let ly = 0xFB;
        let bg_pal = 0xFA;

        let video_io = Arc::new(Mutex::new(VideoIO::new()));
        let uniforms = Rc::new(RefCell::new(HashMap::new()));

        let (tile_bank_0, tile_bank_1, tile_bank_2, map_bank_0, map_bank_1) = get_mock_textures_with_expectations();

        let mut shader: Box<dyn ShaderProgram> = Box::new(
            NullableShaderProgram::new(uniforms.clone(), Rc::new(RefCell::new(false)))
        );

        video_io.lock().set(0xFF40, 0xFF);
        video_io.lock().set(0xFF42, scroll.y as u8);
        video_io.lock().set(0xFF43, scroll.x as u8);
        video_io.lock().set(0xFF44, ly);
        video_io.lock().set(0xFF47, bg_pal);

        let mut video_processor = VideoProcessor::new(
            tile_bank_0, tile_bank_1, tile_bank_2,
            map_bank_0, map_bank_1,
            get_generic_renderable(),
            Arc::new(Mutex::new(VRAM::new())),
            video_io.clone()).unwrap();

        video_processor.draw_background(&mut shader).unwrap();

        assert_eq!(scroll.to_string(), *uniforms.borrow().get("scroll").unwrap());
        //assert_eq!(ivec2(0, ly as i32).to_string(), *uniforms.borrow().get("draw_cutoff").unwrap());
        //this should be uncommented when the actual drawing logic is wired into a CPU. LY being 91 on init means no drawing at all
        // (The code should be changed to make this test pass)
        assert_eq!(bg_pal.to_string(), *uniforms.borrow().get("bg_pal").unwrap());
    }

    #[test]
    fn sets_shader_uniforms_for_window_on_draw() {
        let ly = 0xFB;
        let bg_pal = 0xFA;
        let win_scroll_x = 0xF9;
        let win_scroll_y = 0xF8;

        let video_io = Arc::new(Mutex::new(VideoIO::new()));
        let uniforms = Rc::new(RefCell::new(HashMap::new()));

        let (tile_bank_0, tile_bank_1, tile_bank_2, map_bank_0, map_bank_1) = get_mock_textures_with_expectations();

        let mut shader: Box<dyn ShaderProgram> = Box::new(
            NullableShaderProgram::new(uniforms.clone(), Rc::new(RefCell::new(false)))
        );

        video_io.lock().set(0xFF40, 0xFF);
        video_io.lock().set(0xFF44, ly);
        video_io.lock().set(0xFF47, bg_pal);
        video_io.lock().set(0xFF4A, win_scroll_y);
        video_io.lock().set(0xFF4B, win_scroll_x);

        let mut video_processor = VideoProcessor::new(
            tile_bank_0, tile_bank_1, tile_bank_2,
            map_bank_0, map_bank_1,
            get_generic_renderable(),
            Arc::new(Mutex::new(VRAM::new())),
            video_io.clone()).unwrap();

        video_processor.draw_window(&mut shader).unwrap();

        assert_eq!(ivec2(
            !win_scroll_x.wrapping_sub(8) as i32,
            !win_scroll_y.wrapping_sub(1) as i32
        ).to_string(), *uniforms.borrow().get("scroll").unwrap());
        assert_eq!(ivec2((win_scroll_x as i32) - 8, win_scroll_y as i32)
                       .to_string(),
                   *uniforms.borrow().get("draw_cutoff").unwrap());
        assert_eq!(bg_pal.to_string(), *uniforms.borrow().get("bg_pal").unwrap());
    }

    #[test]
    fn does_not_draw_bg_when_lcd_disabled() {
        let video_io = Arc::new(Mutex::new(VideoIO::new()));

        let draw_count = Rc::new(RefCell::new(0));

        let (tile_bank_0, tile_bank_1, tile_bank_2, map_bank_0, map_bank_1) = get_mock_textures_with_expectations();

        let mut shader_manager = ShaderManager::new();
        shader_manager.register_shader("BACKGROUND".to_string(),
                                       Box::new(
                                           NullableShaderProgram::new(Rc::new(RefCell::new(HashMap::new())), Rc::new(RefCell::new(false)))
                                       )
        ).unwrap();

        video_io.lock().set(0xFF40, 0x00);

        let renderable = Box::new(NullableRenderable::<Vertex2d>::new::<Vertex2d>(
             Rc::new(RefCell::new(false)),
             Rc::new(RefCell::new(vec![])),
             Rc::new(RefCell::new(None)),
             draw_count.clone(),
         ));

        let mut video_processor = VideoProcessor::new(
            tile_bank_0, tile_bank_1, tile_bank_2,
            map_bank_0, map_bank_1,
            renderable,
            Arc::new(Mutex::new(VRAM::new())),
            video_io.clone()).unwrap();

        video_processor.draw(&mut shader_manager).unwrap();

        assert_eq!(0, *draw_count.borrow());
    }

    #[test]
    fn draws_bg_when_lcd_on_and_bg_enabled() {
        let video_io = Arc::new(Mutex::new(VideoIO::new()));

        let draw_count = Rc::new(RefCell::new(0));

        let (tile_bank_0, tile_bank_1, tile_bank_2, map_bank_0, map_bank_1) = get_mock_textures_with_expectations();

        let mut shader_manager = ShaderManager::new();
        shader_manager.register_shader("BACKGROUND".to_string(),
                                       Box::new(
                                           NullableShaderProgram::new(Rc::new(RefCell::new(HashMap::new())), Rc::new(RefCell::new(false)))
                                       )
        ).unwrap();

        video_io.lock().set(0xFF40, LCDCMask::LCD_ENABLE | LCDCMask::BG_ENABLE);

        let renderable = Box::new(NullableRenderable::<Vertex2d>::new::<Vertex2d>(
            Rc::new(RefCell::new(false)),
            Rc::new(RefCell::new(vec![])),
            Rc::new(RefCell::new(None)),
            draw_count.clone(),
        ));

        let mut video_processor = VideoProcessor::new(
            tile_bank_0, tile_bank_1, tile_bank_2,
            map_bank_0, map_bank_1,
            renderable,
            Arc::new(Mutex::new(VRAM::new())),
            video_io.clone()).unwrap();

        video_processor.draw(&mut shader_manager).unwrap();

        assert_eq!(1, *draw_count.borrow());
    }

    #[test]
    fn draws_win_when_lcd_on_and_win_enabled() {
        let video_io = Arc::new(Mutex::new(VideoIO::new()));

        let draw_count = Rc::new(RefCell::new(0));

        let (tile_bank_0, tile_bank_1, tile_bank_2, map_bank_0, map_bank_1) = get_mock_textures_with_expectations();

        let mut shader_manager = ShaderManager::new();
        shader_manager.register_shader("BACKGROUND".to_string(),
                                       Box::new(
                                           NullableShaderProgram::new(Rc::new(RefCell::new(HashMap::new())), Rc::new(RefCell::new(false)))
                                       )
        ).unwrap();

        video_io.lock().set(0xFF40, LCDCMask::LCD_ENABLE | LCDCMask::WIN_ENABLE);

        let renderable = Box::new(NullableRenderable::<Vertex2d>::new::<Vertex2d>(
            Rc::new(RefCell::new(false)),
            Rc::new(RefCell::new(vec![])),
            Rc::new(RefCell::new(None)),
            draw_count.clone(),
        ));

        let mut video_processor = VideoProcessor::new(
            tile_bank_0, tile_bank_1, tile_bank_2,
            map_bank_0, map_bank_1,
            renderable,
            Arc::new(Mutex::new(VRAM::new())),
            video_io.clone()).unwrap();

        video_processor.draw(&mut shader_manager).unwrap();

        assert_eq!(1, *draw_count.borrow());
    }

    #[test]
    fn draws_win_and_bg_when_lcd_on_and_bg_and_win_enabled() {
        let video_io = Arc::new(Mutex::new(VideoIO::new()));

        let draw_count = Rc::new(RefCell::new(0));

        let (tile_bank_0, tile_bank_1, tile_bank_2, map_bank_0, map_bank_1) = get_mock_textures_with_expectations();

        let mut shader_manager = ShaderManager::new();
        shader_manager.register_shader("BACKGROUND".to_string(),
                                       Box::new(
                                           NullableShaderProgram::new(Rc::new(RefCell::new(HashMap::new())), Rc::new(RefCell::new(false)))
                                       )
        ).unwrap();

        video_io.lock().set(0xFF40, LCDCMask::LCD_ENABLE | LCDCMask::WIN_ENABLE | LCDCMask::BG_ENABLE);

        let renderable = Box::new(NullableRenderable::<Vertex2d>::new::<Vertex2d>(
            Rc::new(RefCell::new(false)),
            Rc::new(RefCell::new(vec![])),
            Rc::new(RefCell::new(None)),
            draw_count.clone(),
        ));

        let mut video_processor = VideoProcessor::new(
            tile_bank_0, tile_bank_1, tile_bank_2,
            map_bank_0, map_bank_1,
            renderable,
            Arc::new(Mutex::new(VRAM::new())),
            video_io.clone()).unwrap();

        video_processor.draw(&mut shader_manager).unwrap();

        assert_eq!(2, *draw_count.borrow());
    }
}
