use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;
use dec_gl::{GLHandler, UICamera, Vertex2d};
use dec_gl::framebuffer::SimpleFramebuffer;
use dec_gl::renderable::GlRenderable;
use dec_gl::shader::{GLShaderProgram, ShaderManager};
use mockall_double::double;
#[double]
use dec_gl::texture::{Texture2Du8, Texture3Du8};
use dec_gl::types::{ivec2, vec4, Vec3};
use dialog::{DialogBox, FileSelection};
use parking_lot::Mutex;
use crate::cpu::GameBoyCPU;
use crate::memory::MemoryController;
use crate::renderer::VideoProcessor;
use crate::system::MainBoard;

pub struct App {
    pub _args: Vec<String>,

    camera: UICamera,

    gl_handler: Rc<RefCell<GLHandler>>,
    framebuffer: SimpleFramebuffer,
    shader_manager: ShaderManager
}

const GB_COLUR_0: Vec3 = Vec3{x: 0.7, y: 1.0, z: 0.5};
const GB_COLUR_1: Vec3 = Vec3{x: 0.45, y: 0.7, z: 0.3};
const GB_COLUR_2: Vec3 = Vec3{x: 0.3, y: 0.45, z: 0.15};
const GB_COLUR_3: Vec3 = Vec3{x: 0.2, y: 0.3, z: 0.0};


impl App {
     pub fn new(args: Vec<String>, gl_handler: Rc<RefCell<GLHandler>>) -> App {
         let window_size = gl_handler.borrow().get_window().get_window_size();

         let mut shader_manager = ShaderManager::new();

         shader_manager.register_shader(
             "UI".to_string(),
             Box::new(GLShaderProgram::load_shader_program("assets/graphics/shaders/ui", "UI", false).unwrap())
         ).unwrap();

         let framebuffer = SimpleFramebuffer::new(window_size.x as i32, window_size.y as i32).unwrap();

         let camera = UICamera::new(ivec2(160, 144), -1.0, 1.0);


        App {
            _args: args,

            camera,

            gl_handler,
            framebuffer,
            shader_manager
        }
     }



    pub fn run (&mut self) {
        self.framebuffer.set_clear_colour(vec4(GB_COLUR_0.x, GB_COLUR_0.y, GB_COLUR_0.z, 1.0));

        match self.shader_manager.bind("UI".to_string()) {
            Ok(shader) => {
                shader.set_uniform("pv".to_string(), &self.camera.get_matrix());

                shader.set_uniform("bgMap".to_string(), &0);
                shader.set_uniform("tileMapBank0".to_string(), &1);
                shader.set_uniform("tileMapBank1".to_string(), &2);

                shader.set_uniform("gbColour0".to_string(), &GB_COLUR_0);
                shader.set_uniform("gbColour1".to_string(), &GB_COLUR_1);
                shader.set_uniform("gbColour2".to_string(), &GB_COLUR_2);
                shader.set_uniform("gbColour3".to_string(), &GB_COLUR_3);

                shader
            }
            Err(_) => return
        };

        let memory_controller = Arc::new(Mutex::new(MemoryController::new()));
        let cpu = Box::new(GameBoyCPU::new_with_nop());

        let rom_path = match self.get_rom_path() {
            Some(path) => path,
            None => return
        };
        memory_controller.lock().load_rom(&rom_path);

        let video_processor = {
            let vram = memory_controller.lock().get_vram_arc();
            let video_io = memory_controller.lock().get_io_map().lock().get_video_io();

            VideoProcessor::new(
                Texture3Du8::default(),
                Texture3Du8::default(),
                Texture3Du8::default(),

                Texture2Du8::default(),
                Texture2Du8::default(),

                Box::new(GlRenderable::<Vertex2d>::new::<Vertex2d>()),

                vram,
                video_io,
            ).unwrap()
        };

        let mut main_board = MainBoard::new(
            cpu,
            memory_controller,
            video_processor
        );

        let mut _frame: u64 = 0;

        while !self.gl_handler.borrow().wind_should_close() {
            self.framebuffer.clear();

            for event in self.gl_handler.borrow_mut().handle_events() {
                match event {
                    _ => { },
                }
            }

            if self.gl_handler.borrow().get_window().has_resized_this_frame() { self.resize(); }

            self.framebuffer.bind_draw_target();


            match self.shader_manager.bind("UI".to_string()) {
                Ok(shader) => {
                    main_board.perform_frame(shader)
                }
                Err(_) => return
            };

            SimpleFramebuffer::bind_default_framebuffer();
            self.framebuffer.blit(
                self.gl_handler.borrow().get_window().get_window_size(),
                gl::COLOR_BUFFER_BIT,
                gl::NEAREST,
            );

            self.gl_handler.borrow_mut().poll_window();

            _frame += 1;
        }
    }

    fn resize(&mut self) {
        let window_size = self.gl_handler.borrow().get_window().get_window_size();

        self.framebuffer.resize(window_size.x, window_size.y);
    }

    fn get_rom_path(&self) -> Option<String> {
        FileSelection::new("Open ROM File")
            .title("Rom File")
            .show()
            .unwrap()
    }
}
