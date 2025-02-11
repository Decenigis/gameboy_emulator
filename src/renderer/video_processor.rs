use std::sync::Arc;
use mockall_double::double;
use dec_gl::{Renderable, Vertex2d};
use dec_gl::shader::ShaderProgram;
#[double]
use dec_gl::texture::{Texture2Du8, Texture3Du8};
use dec_gl::types::{ivec2, ivec3};
use parking_lot::{Mutex, MutexGuard};
use crate::memory::io_map::VideoIO;
use crate::memory::VRAM;
use crate::renderer::RendererError;

pub struct VideoProcessor {
    tilemap_bank_0: Texture3Du8,
    tilemap_bank_1: Texture3Du8,
    tilemap_bank_2: Texture3Du8,

    map_bank_0: Texture2Du8,
    map_bank_1: Texture2Du8,

    background_renderable: Renderable,
    
    vram: Arc<Mutex<VRAM>>,
    video_io: Arc<Mutex<VideoIO>>
}

impl VideoProcessor {
    
    pub fn new(
        tilemap_bank_0: Texture3Du8,
        tilemap_bank_1: Texture3Du8,
        tilemap_bank_2: Texture3Du8,

        map_bank_0: Texture2Du8,
        map_bank_1: Texture2Du8,

        mut background_renderable: Renderable,

        vram: Arc<Mutex<VRAM>>,
        video_io: Arc<Mutex<VideoIO>>
    )
        -> Result<VideoProcessor, RendererError> 
    {
        let _result = background_renderable.update_data(&vec![
            Vertex2d { x: 0.0, y: 0.0, u: 0.0, v: 0.0},
            Vertex2d { x: 0.0, y: 144.0, u: 0.0, v: 1.0},
            Vertex2d { x: 160.0, y: 0.0, u: 1.0, v: 0.0},

            Vertex2d { x: 0.0, y: 144.0, u: 0.0, v: 1.0},
            Vertex2d { x: 160.0, y: 0.0, u: 1.0, v: 0.0},
            Vertex2d { x: 160.0, y: 144.0, u: 1.0, v: 1.0}],
        None);

        #[cfg(not(test))]
        match _result {
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

    fn bind_textures_to_units(&self, lcd_ctrl: u8) {
        if lcd_ctrl & 0x10 == 0 {
            self.tilemap_bank_2.bind_to_unit(1);
        }
        else {
            self.tilemap_bank_0.bind_to_unit(1);
        }

        self.tilemap_bank_1.bind_to_unit(2);


        if lcd_ctrl & 0x08 == 0 {
            self.map_bank_0.bind_to_unit(0);
        }
        else {
            self.map_bank_1.bind_to_unit(0);
        }
    }

    fn set_background_shader_values(&mut self, shader: &mut Box<dyn ShaderProgram>, video_io_mutex: MutexGuard<VideoIO>){
        shader.bind();
        shader.set_uniform("lcd_ctrl".to_string(), &(video_io_mutex.get_lcd_ctrl() as i32));
        shader.set_uniform("lcd_stat".to_string(), &(video_io_mutex.get_lcd_stat() as i32));
        shader.set_uniform("scroll".to_string(), &ivec2(video_io_mutex.get_scroll_x() as i32, video_io_mutex.get_scroll_y() as i32));
        shader.set_uniform("ly".to_string(), &(video_io_mutex.get_ly() as i32));
        shader.set_uniform("bg_pal".to_string(), &(video_io_mutex.get_bg_pal() as i32));
    }
    
    pub fn draw(&mut self, shader: &mut Box<dyn ShaderProgram>) -> Result<(), RendererError> {
        {
            let video_io_mutex = self.video_io.clone();
            let video_io_guard = video_io_mutex.lock();
            let lcd_ctrl = video_io_guard.get_lcd_ctrl().clone();

            if video_io_guard.get_lcd_ctrl() & 0x80 == 0 {
                return Ok(());
            }

            self.set_background_shader_values(shader, video_io_guard);
            self.bind_textures_to_units(lcd_ctrl);
        }

        self.background_renderable.draw();

        Ok(())
    }
}


#[cfg(test)]
mod video_processor_tests {
    use std::cell::RefCell;
    use std::collections::HashMap;
    use std::rc::Rc;
    use std::sync::Arc;
    use dec_gl::Renderable;
    use dec_gl::shader::{NullableShaderProgram, ShaderProgram};
    use dec_gl::texture::{MockTexture2Du8, MockTexture3Du8};
    use dec_gl::types::ivec2;
    use mockall::predicate::eq;
    use parking_lot::Mutex;
    use crate::memory::io_map::VideoIO;
    use crate::memory::{MemoryTrait, VRAM};
    use crate::renderer::VideoProcessor;


    fn get_mock_textures() -> (MockTexture3Du8, MockTexture3Du8, MockTexture3Du8, MockTexture2Du8, MockTexture2Du8) {
        let mock_texture_3d_0 = MockTexture3Du8::default();
        let mock_texture_3d_1 = MockTexture3Du8::default();
        let mock_texture_3d_2 = MockTexture3Du8::default();
        let mock_texture_2d_0 = MockTexture2Du8::default();
        let mock_texture_2d_1 = MockTexture2Du8::default();
        (mock_texture_3d_0, mock_texture_3d_1, mock_texture_3d_2, mock_texture_2d_0, mock_texture_2d_1)
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
            Renderable::new_uninitialised(),
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
            Renderable::new_uninitialised(),
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
            Renderable::new_uninitialised(),
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
            Renderable::new_uninitialised(),
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
            Renderable::new_uninitialised(),
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
            Renderable::new_uninitialised(),
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
        tile_bank_2.expect_bind_to_unit().with(eq(1)).times(1).returning(|_| ());

        let video_processor = VideoProcessor::new(
            tile_bank_0, tile_bank_1, tile_bank_2,
            map_bank_0, map_bank_1,
            Renderable::new_uninitialised(),
            Arc::new(Mutex::new(VRAM::new())),
            Arc::new(Mutex::new(VideoIO::new()))).unwrap();

        video_processor.bind_textures_to_units(0x80);
    }


    #[test]
    fn binds_tile_map_0_given_lcdc_bit_4_set() {
        let (mut tile_bank_0, mut tile_bank_1, mut tile_bank_2, mut map_bank_0, map_bank_1) = get_mock_textures();
        tile_bank_1.expect_bind_to_unit().returning(|_| ());
        map_bank_0.expect_bind_to_unit().returning(|_| ());

        tile_bank_0.expect_bind_to_unit().with(eq(1)).times(1).returning(|_| ());
        tile_bank_2.expect_bind_to_unit().times(0).returning(|_| ());

        let video_processor = VideoProcessor::new(
            tile_bank_0, tile_bank_1, tile_bank_2,
            map_bank_0, map_bank_1,
            Renderable::new_uninitialised(),
            Arc::new(Mutex::new(VRAM::new())),
            Arc::new(Mutex::new(VideoIO::new()))).unwrap();

        video_processor.bind_textures_to_units(0x90);
    }

    #[test]
    fn binds_tile_map_1_whether_bit_4_is_or_is_not_set() {
        let (mut tile_bank_0, mut tile_bank_1, mut tile_bank_2, mut map_bank_0, map_bank_1) = get_mock_textures();
        tile_bank_0.expect_bind_to_unit().returning(|_| ());
        tile_bank_2.expect_bind_to_unit().returning(|_| ());
        map_bank_0.expect_bind_to_unit().returning(|_| ());

        tile_bank_1.expect_bind_to_unit().with(eq(2)).times(2).returning(|_| ());

        let video_processor = VideoProcessor::new(
            tile_bank_0, tile_bank_1, tile_bank_2,
            map_bank_0, map_bank_1,
            Renderable::new_uninitialised(),
            Arc::new(Mutex::new(VRAM::new())),
            Arc::new(Mutex::new(VideoIO::new()))).unwrap();

        video_processor.bind_textures_to_units(0x80);
        video_processor.bind_textures_to_units(0x90);
    }

    #[test]
    fn binds_map_bank_0_given_lcdc_bit_3_not_set() {
        let (tile_bank_0, mut tile_bank_1, mut tile_bank_2, mut map_bank_0, mut map_bank_1) = get_mock_textures();
        tile_bank_1.expect_bind_to_unit().returning(|_| ());
        tile_bank_2.expect_bind_to_unit().returning(|_| ());

        map_bank_0.expect_bind_to_unit().with(eq(0)).times(1).returning(|_| ());
        map_bank_1.expect_bind_to_unit().times(0).returning(|_| ());

        let video_processor = VideoProcessor::new(
            tile_bank_0, tile_bank_1, tile_bank_2,
            map_bank_0, map_bank_1,
            Renderable::new_uninitialised(),
            Arc::new(Mutex::new(VRAM::new())),
            Arc::new(Mutex::new(VideoIO::new()))).unwrap();

        video_processor.bind_textures_to_units(0x80);
    }

    #[test]
    fn binds_map_bank_1_given_lcdc_bit_3_set() {
        let (tile_bank_0, mut tile_bank_1, mut tile_bank_2, mut map_bank_0, mut map_bank_1) = get_mock_textures();
        tile_bank_1.expect_bind_to_unit().returning(|_| ());
        tile_bank_2.expect_bind_to_unit().returning(|_| ());

        map_bank_0.expect_bind_to_unit().times(0).returning(|_| ());
        map_bank_1.expect_bind_to_unit().with(eq(0)).times(1).returning(|_| ());

        let video_processor = VideoProcessor::new(
            tile_bank_0, tile_bank_1, tile_bank_2,
            map_bank_0, map_bank_1,
            Renderable::new_uninitialised(),
            Arc::new(Mutex::new(VRAM::new())),
            Arc::new(Mutex::new(VideoIO::new()))).unwrap();

        video_processor.bind_textures_to_units(0x88);
    }

    #[test]
    fn binds_the_shader_on_draw() {
        let vram = Arc::new(Mutex::new(VRAM::new()));
        let video_io = Arc::new(Mutex::new(VideoIO::new()));
        let was_bound = Rc::new(RefCell::new(false));
        vram.lock().set_not_stale();

        let (tile_bank_0, mut tile_bank_1, mut tile_bank_2, mut map_bank_0, map_bank_1) = get_mock_textures();
        tile_bank_1.expect_bind_to_unit().returning(|_| ());
        tile_bank_2.expect_bind_to_unit().returning(|_| ());
        map_bank_0.expect_bind_to_unit().returning(|_| ());

        let mut shader: Box<dyn ShaderProgram> = Box::new(
            NullableShaderProgram::new(Rc::new(RefCell::new(HashMap::new())), was_bound.clone()),
        );

        video_io.lock().set(0xFF40, 0x80);

        let mut video_processor = VideoProcessor::new(
            tile_bank_0, tile_bank_1, tile_bank_2,
            map_bank_0, map_bank_1,
            Renderable::new_uninitialised(),
            vram.clone(),
            video_io.clone()).unwrap();

        video_processor.draw(&mut shader).unwrap();

        assert_eq!(true, *was_bound.borrow());
    }

    #[test]
    fn sets_shader_uniforms_on_draw() {
        let lcd_ctrl_value = 0xFF;
        let lcd_stat_value = 0xFE;
        let scroll = ivec2(0xFD, 0xFC);
        let ly = 0xFB;
        let bg_pal = 0xFA;

        let video_io = Arc::new(Mutex::new(VideoIO::new()));
        let uniforms = Rc::new(RefCell::new(HashMap::new()));

        let (mut tile_bank_0, mut tile_bank_1, mut tile_bank_2, mut map_bank_0, mut map_bank_1) = get_mock_textures();
        tile_bank_0.expect_bind_to_unit().returning(|_| ());
        tile_bank_1.expect_bind_to_unit().returning(|_| ());
        tile_bank_2.expect_bind_to_unit().returning(|_| ());
        map_bank_0.expect_bind_to_unit().returning(|_| ());
        map_bank_1.expect_bind_to_unit().returning(|_| ());

        let mut shader: Box<dyn ShaderProgram> = Box::new(
            NullableShaderProgram::new(uniforms.clone(), Rc::new(RefCell::new(false)))
        );

        video_io.lock().set(0xFF40, lcd_ctrl_value);
        video_io.lock().set(0xFF41, lcd_stat_value);
        video_io.lock().set(0xFF42, scroll.y as u8);
        video_io.lock().set(0xFF43, scroll.x as u8);
        video_io.lock().set(0xFF44, ly);
        video_io.lock().set(0xFF47, bg_pal);

        let mut video_processor = VideoProcessor::new(
            tile_bank_0, tile_bank_1, tile_bank_2,
            map_bank_0, map_bank_1,
            Renderable::new_uninitialised(),
            Arc::new(Mutex::new(VRAM::new())),
            video_io.clone()).unwrap();

        video_processor.draw(&mut shader).unwrap();

        assert_eq!(*uniforms.borrow().get("lcd_ctrl").unwrap(), lcd_ctrl_value.to_string());
        assert_eq!(*uniforms.borrow().get("lcd_stat").unwrap(), lcd_stat_value.to_string());
        assert_eq!(*uniforms.borrow().get("scroll").unwrap(), scroll.to_string());
        assert_eq!(*uniforms.borrow().get("ly").unwrap(), ly.to_string());
        assert_eq!(*uniforms.borrow().get("bg_pal").unwrap(), bg_pal.to_string());
    }
}
