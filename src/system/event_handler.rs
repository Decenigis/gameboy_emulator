use std::sync::Arc;
use dec_gl::shader::ShaderManager;
use parking_lot::Mutex;
use crate::app::PerformanceTimer;
use crate::cpu::{Interrupt, CPU};
use crate::memory::MemoryController;
use crate::renderer::VideoProcessor;
use crate::system::clock_event::ClockEvent;
use crate::system::system_error::SystemError;

pub struct EventHandler {}

impl EventHandler {

    pub fn new() -> Self {
        Self {}
    }

    pub fn handle_event(&mut self,
                        cpu: &mut Box<dyn CPU>,
                        memory: Arc<Mutex<MemoryController>>,
                        video_processor: &mut VideoProcessor,
                        shader_manager: &mut ShaderManager,
                        event: &ClockEvent,
                        performance_timer: &mut PerformanceTimer
    )
    -> Result<bool, SystemError>
    {
        match event {
            ClockEvent::CPUClock => {
                performance_timer.set_category("CPU");
                memory.lock().clock();

                cpu.clock(memory.clone());
            }
            ClockEvent::DrawLine => {
                performance_timer.set_category("Draw");
                video_processor.try_update_graphics_data();

                match video_processor.draw(shader_manager) {
                    Ok(_) => {}
                    Err(error) => return Err(SystemError::RendererError { error }),
                }
            }
            ClockEvent::SendFrame => {
                performance_timer.set_category("Draw");
                return Ok(true)
            }
            ClockEvent::VBlankInterrupt => {
                performance_timer.set_category("CPU");
                cpu.try_interrupt(memory.clone(), Interrupt::VBlank);
            }
        }
        Ok(false)
    }
}



#[cfg(test)]
mod tests { //these are not very nice
    use std::cell::RefCell;
    use std::collections::HashMap;
    use std::rc::Rc;
    use dec_gl::renderable::{NullableRenderable, Renderable};
    use dec_gl::shader::NullableShaderProgram;
    use dec_gl::texture::{MockTexture2Du8, MockTexture3Du8};
    use dec_gl::Vertex2d;
    use crate::cpu::NullableCPU;
    use crate::memory::MemoryTrait;
    use super::*;

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

        mock_texture_3d_0.expect_set_data().returning(|_, _| Ok(()));
        mock_texture_3d_1.expect_set_data().returning(|_, _| Ok(()));
        mock_texture_3d_2.expect_set_data().returning(|_, _| Ok(()));
        mock_texture_2d_0.expect_set_data().returning(|_, _| Ok(()));
        mock_texture_2d_1.expect_set_data().returning(|_, _| Ok(()));

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
    fn cpu_clock_event_clocks_cpu() {
        let mut event_handler = EventHandler::new();
        let number_of_times_clocked = Rc::new(RefCell::new(0));
        let mut cpu: Box<dyn CPU> = Box::new(NullableCPU::new(number_of_times_clocked.clone(), Rc::new(RefCell::new(None))));
        let memory = Arc::new(Mutex::new(MemoryController::new()));

        let (tile_bank_0, tile_bank_1, tile_bank_2, map_bank_0, map_bank_1) = get_mock_textures_with_expectations();

        let vram = memory.lock().get_vram_arc();
        let oam = memory.lock().get_oam_arc();
        let video_io = memory.lock().get_io_map().lock().get_video_io();

        let mut video_processor = VideoProcessor::new(
            tile_bank_0, tile_bank_1, tile_bank_2,
            map_bank_0, map_bank_1,
            get_generic_renderable(),
            get_generic_renderable(),
            vram,
            oam,
            video_io).unwrap();

        let mut shader_manager = ShaderManager::new();

        let event = ClockEvent::CPUClock;

        event_handler.handle_event(&mut cpu, memory.clone(), &mut video_processor, &mut shader_manager, &event, &mut PerformanceTimer::new_fake()).unwrap();

        assert_eq!(1, *number_of_times_clocked.borrow());
    }

    #[test]
    fn draw_line_event_draws_line() {
        let mut event_handler = EventHandler::new();
        let mut cpu: Box<dyn CPU> = Box::new(NullableCPU::new( Rc::new(RefCell::new(0)),  Rc::new(RefCell::new(None))));
        let memory = Arc::new(Mutex::new(MemoryController::new()));

        let (tile_bank_0, tile_bank_1, tile_bank_2, map_bank_0, map_bank_1) = get_mock_textures_with_expectations();

        let vram = memory.lock().get_vram_arc();
        let oam = memory.lock().get_oam_arc();
        let video_io = memory.lock().get_io_map().lock().get_video_io();

        video_io.lock().set(0xFF40, 0x81);

        let draw_count = Rc::new(RefCell::new(0));

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
            get_generic_renderable(),
            vram,
            oam,
            video_io).unwrap();

        let mut shader_manager = ShaderManager::new();
        shader_manager.register_shader("BACKGROUND".to_string(), Box::new(NullableShaderProgram::new(Rc::new(RefCell::new(HashMap::new())), Rc::new(RefCell::new(false))))).unwrap();

        let event = ClockEvent::DrawLine;

        event_handler.handle_event(&mut cpu, memory.clone(), &mut video_processor, &mut shader_manager, &event, &mut PerformanceTimer::new_fake()).unwrap();

        assert_eq!(1, draw_count.borrow().clone());
    }

    #[test]
    fn send_frame_returns_true() {
        let mut event_handler = EventHandler::new();
        let mut cpu: Box<dyn CPU> = Box::new(NullableCPU::new( Rc::new(RefCell::new(0)),  Rc::new(RefCell::new(None))));
        let memory = Arc::new(Mutex::new(MemoryController::new()));

        let (tile_bank_0, tile_bank_1, tile_bank_2, map_bank_0, map_bank_1) = get_mock_textures_with_expectations();

        let vram = memory.lock().get_vram_arc();
        let oam = memory.lock().get_oam_arc();
        let video_io = memory.lock().get_io_map().lock().get_video_io();

        let mut video_processor = VideoProcessor::new(
            tile_bank_0, tile_bank_1, tile_bank_2,
            map_bank_0, map_bank_1,
            get_generic_renderable(),
            get_generic_renderable(),
            vram,
            oam,
            video_io).unwrap();

        let mut shader_manager = ShaderManager::new();

        let event = ClockEvent::SendFrame;

        let send_frame = event_handler.handle_event(&mut cpu, memory.clone(), &mut video_processor, &mut shader_manager, &event, &mut PerformanceTimer::new_fake()).unwrap();

        assert_eq!(true, send_frame);
    }

    #[test]
    fn vblank_performs_vblank_interrupt() {
        let mut event_handler = EventHandler::new();
        let interrupt = Rc::new(RefCell::new(None));
        let mut cpu: Box<dyn CPU> = Box::new(NullableCPU::new( Rc::new(RefCell::new(0)),  interrupt.clone()));
        let memory = Arc::new(Mutex::new(MemoryController::new()));

        let (tile_bank_0, tile_bank_1, tile_bank_2, map_bank_0, map_bank_1) = get_mock_textures_with_expectations();

        let vram = memory.lock().get_vram_arc();
        let oam = memory.lock().get_oam_arc();
        let video_io = memory.lock().get_io_map().lock().get_video_io();

        let mut video_processor = VideoProcessor::new(
            tile_bank_0, tile_bank_1, tile_bank_2,
            map_bank_0, map_bank_1,
            get_generic_renderable(),
            get_generic_renderable(),
            vram,
            oam,
            video_io).unwrap();

        let mut shader_manager = ShaderManager::new();

        let event = ClockEvent::VBlankInterrupt;

        event_handler.handle_event(&mut cpu, memory.clone(), &mut video_processor, &mut shader_manager, &event, &mut PerformanceTimer::new_fake()).unwrap();

        assert_eq!(Some(Interrupt::VBlank), *interrupt.borrow());
    }
}
