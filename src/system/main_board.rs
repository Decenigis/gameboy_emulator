use std::sync::Arc;
use dec_gl::shader::ShaderProgram;
use parking_lot::Mutex;
use crate::cpu::CPU;
use crate::memory::MemoryController;
use crate::renderer::VideoProcessor;
use crate::system::event_handler::EventHandler;
use crate::system::vdu_counter::VDUCounter;

pub struct MainBoard {
    cpu: Box<dyn CPU>,
    vdu_counter: VDUCounter,
    memory: Arc<Mutex<MemoryController>>,
    video_processor: VideoProcessor,
    event_handler: EventHandler,
}

impl MainBoard {

    pub fn new(cpu: Box<dyn CPU>, memory: Arc<Mutex<MemoryController>>, video_processor: VideoProcessor) -> Self {
        Self {
            cpu,
            vdu_counter: VDUCounter::new(memory.clone().lock().get_io_map().lock().get_video_io()),
            memory,
            video_processor,
            event_handler: EventHandler::new(),
        }
    }

    pub fn perform_frame(&mut self, bacgkround_shader: &mut Box<dyn ShaderProgram>) {
        let mut send_frame = false;

        while !send_frame {
            let events = self.vdu_counter.tick();

            for event in events {
                send_frame = send_frame || self.event_handler.handle_event(
                    &mut self.cpu,
                    self.memory.clone(),
                    &mut self.video_processor,
                    bacgkround_shader,
                    &event);
            }
        }
    }
}
