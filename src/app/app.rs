use std::cell::RefCell;
use std::fs::File;
use std::io::Write;
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
use glfw::{Action, Key, WindowEvent};
use parking_lot::Mutex;
use crate::cpu::GameBoyCPU;
use crate::memory::{MemoryController, MemoryTrait};
use crate::renderer::VideoProcessor;
use crate::system::MainBoard;

pub struct App {
    pub _args: Vec<String>,

    camera: UICamera,

    gl_handler: Rc<RefCell<GLHandler>>,
    framebuffer: SimpleFramebuffer,
    shader_manager: ShaderManager,

    rom_path: Option<String>
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
             "BACKGROUND".to_string(),
             Box::new(GLShaderProgram::load_shader_program("assets/graphics/shaders/background", "BACKGROUND", false).unwrap())
         ).unwrap();
         shader_manager.register_shader(
             "OBJECT".to_string(),
             Box::new(GLShaderProgram::load_shader_program("assets/graphics/shaders/object", "OBJECT", false).unwrap())
         ).unwrap();

         let framebuffer = SimpleFramebuffer::new(window_size.x as i32, window_size.y as i32).unwrap();

         let camera = UICamera::new(ivec2(160, 144), -1.0, 1.0);


        App {
            _args: args,

            camera,

            gl_handler,
            framebuffer,
            shader_manager,
            rom_path: None
        }
     }



    pub fn run (&mut self) {
        self.framebuffer.set_clear_colour(vec4(GB_COLUR_0.x, GB_COLUR_0.y, GB_COLUR_0.z, 1.0));

        match self.shader_manager.bind("BACKGROUND".to_string()) {
            Ok(shader) => {
                shader.set_uniform("pv".to_string(), &self.camera.get_matrix());

                shader.set_uniform("tileMapBank0".to_string(), &0);
                shader.set_uniform("tileMapBank1".to_string(), &1);
                shader.set_uniform("bgMap".to_string(), &2);

                shader.set_uniform("gbColour0".to_string(), &GB_COLUR_0);
                shader.set_uniform("gbColour1".to_string(), &GB_COLUR_1);
                shader.set_uniform("gbColour2".to_string(), &GB_COLUR_2);
                shader.set_uniform("gbColour3".to_string(), &GB_COLUR_3);

                shader
            }
            Err(_) => return
        };


        match self.shader_manager.bind("OBJECT".to_string()) {
            Ok(shader) => {
                shader.set_uniform("pv".to_string(), &self.camera.get_matrix());

                shader.set_uniform("tileMapBank0".to_string(), &0);
                shader.set_uniform("tileMapBank1".to_string(), &1);

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

        self.rom_path = self.get_rom_path();
        match &self.rom_path {
            Some(path) => memory_controller.lock().load_rom(path),
            None => {}
        };

        let video_processor = {
            let vram = memory_controller.lock().get_vram_arc();
            let oam = memory_controller.lock().get_oam_arc();
            let video_io = memory_controller.lock().get_io_map().lock().get_video_io();

            VideoProcessor::new(
                Texture3Du8::default(),
                Texture3Du8::default(),
                Texture3Du8::default(),

                Texture2Du8::default(),
                Texture2Du8::default(),

                Box::new(GlRenderable::<Vertex2d>::new::<Vertex2d>()),
                Box::new(GlRenderable::<Vertex2d>::new::<Vertex2d>()),

                vram,
                oam,
                video_io,
            ).unwrap()
        };

        let mut main_board = MainBoard::new(
            cpu,
            memory_controller.clone(),
            video_processor
        );

        let joypad = memory_controller.lock().get_io_map().lock().get_joypad_io();

        let mut _frame: u64 = 0;

        let mut last_frame = std::time::Instant::now();
        let mut now = std::time::Instant::now();

        while !self.gl_handler.borrow().wind_should_close() {
            self.framebuffer.clear();

            let events = self.gl_handler.borrow_mut().handle_events();
            for event in events.clone() {
                match event {
                    WindowEvent::Key(Key::F12, _, Action::Press, _) => {
                        let mut file = File::create("memdump.bin").unwrap();
                        let mut memory: Vec<u8> = vec![];

                        for i in 0..0xFFFF {
                            memory.push(memory_controller.lock().get(i));
                        }

                        file.write_all(memory.as_slice()).unwrap(); },

                    WindowEvent::Key(Key::W, _, action, _) => {
                        if action == Action::Press || action == Action::Repeat {
                            joypad.lock().set_up(true);
                        } else {
                            joypad.lock().set_up(false);
                        }
                    }
                    WindowEvent::Key(Key::A, _, action, _) => {
                        if action == Action::Press || action == Action::Repeat {
                            joypad.lock().set_left(true);
                        } else {
                            joypad.lock().set_left(false);
                        }
                    }
                    WindowEvent::Key(Key::S, _, action, _) => {
                        if action == Action::Press || action == Action::Repeat {
                            joypad.lock().set_down(true);
                        } else {
                            joypad.lock().set_down(false);
                        }
                    }
                    WindowEvent::Key(Key::D, _, action, _) => {
                        if action == Action::Press || action == Action::Repeat {
                            joypad.lock().set_right(true);
                        } else {
                            joypad.lock().set_right(false);
                        }
                    }
                    WindowEvent::Key(Key::Space, _, action, _) => {
                        if action == Action::Press || action == Action::Repeat {
                            joypad.lock().set_a(true);
                        } else {
                            joypad.lock().set_a(false);
                        }
                    }
                    WindowEvent::Key(Key::Enter, _, action, _) => {
                        if action == Action::Press || action == Action::Repeat {
                            joypad.lock().set_b(true);
                        } else {
                            joypad.lock().set_b(false);
                        }
                    }
                    WindowEvent::Key(Key::Q, _, action, _) => {
                        if action == Action::Press || action == Action::Repeat {
                            joypad.lock().set_start(true);
                        } else {
                            joypad.lock().set_start(false);
                        }
                    }
                    WindowEvent::Key(Key::E, _, action, _) => {
                        if action == Action::Press || action == Action::Repeat {
                            joypad.lock().set_select(true);
                        } else {
                            joypad.lock().set_select(false);
                        }
                    }
                    WindowEvent::Key(Key::V, _, Action::Press, _) => {
                        let new_vsync = { !self.gl_handler.borrow().get_vsync() };
                        self.gl_handler.borrow_mut().set_vsync(new_vsync);
                    }
                    WindowEvent::Key(Key::R, _, Action::Press, _) => {
                        main_board.reset().unwrap();
                        match &self.rom_path {
                            Some(path) => memory_controller.lock().load_rom(path),
                            None => {}
                        };
                    }
                    WindowEvent::Key(Key::L, _, Action::Press, _) => {
                        self.rom_path = self.get_rom_path();

                        main_board.reset().unwrap();
                        match &self.rom_path {
                            Some(path) => memory_controller.lock().load_rom(path),
                            None => {}
                        };
                    }
                    _ => {}
                }
            }

            if self.gl_handler.borrow().get_window().has_resized_this_frame() { self.resize(); }

            self.framebuffer.bind_draw_target();


            main_board.perform_frame(&mut self.shader_manager).unwrap();

            SimpleFramebuffer::bind_default_framebuffer();
            self.framebuffer.blit(
                self.gl_handler.borrow().get_window().get_window_size(),
                gl::COLOR_BUFFER_BIT,
                gl::NEAREST,
            );

            self.gl_handler.borrow_mut().poll_window();

            now = std::time::Instant::now();
            let elapsed = now.duration_since(last_frame);
            last_frame = now;

            self.gl_handler.borrow_mut().get_window_mut().set_title(format!("GB EMULATOR - {} FPS", (1.0 / elapsed.as_secs_f64()).round()).as_str());

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
