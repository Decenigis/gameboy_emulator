use std::cell::RefCell;
use std::rc::Rc;
use dec_gl::{FrameBuffer, GLHandler, Renderable, UICamera, Vertex2d};
use dec_gl::shader::{ShaderManager, ShaderProgram};
use dec_gl::texture::{Texture2D, TextureManager};
use glm::ivec2;

pub struct App {
    pub _args: Vec<String>,

    camera: UICamera,

    gl_handler: Rc<RefCell<GLHandler>>,
    framebuffer: FrameBuffer,
    texture_manager: TextureManager,
    shader_manager: ShaderManager
}


impl App {
     pub fn new(args: Vec<String>, gl_handler: Rc<RefCell<GLHandler>>) -> App {
         let window_size = gl_handler.borrow().get_window().get_window_size();

         let mut shader_manager = ShaderManager::new();
         let mut texture_manager = TextureManager::new();

         shader_manager.register_shader("UI".to_string(), ShaderProgram::load_shader_program("assets/graphics/shaders/ui", "UI", false)).unwrap();

         texture_manager.register_texture("test".to_string(), Texture2D::new("assets/graphics/textures/no_texture.png".as_ref(), false)).unwrap();

         let framebuffer = FrameBuffer::new(window_size.x as i32, window_size.y as i32).unwrap();

         let camera = UICamera::new(ivec2(160, 144), -1.0, 1.0);


        App {
            _args: args,

            camera,

            gl_handler,
            framebuffer,
            texture_manager,
            shader_manager
        }
     }

    pub fn run (&mut self) {
        match self.shader_manager.bind("UI".to_string()) {
            Ok(shader) => {
                shader.set_uniform("pv".to_string(), self.camera.get_matrix());
                shader
            }
            Err(_) => return
        };
        self.texture_manager.bind("test".to_string());

        let renderable = Renderable::new_initialised(&vec![
            Vertex2d { x: 0.0, y: 0.0, u: 0.0, v: 0.0},
            Vertex2d { x: 0.0, y: 144.0, u: 0.0, v: 1.0},
            Vertex2d { x: 160.0, y: 0.0, u: 1.0, v: 0.0},

            Vertex2d { x: 0.0, y: 144.0, u: 0.0, v: 1.0},
            Vertex2d { x: 160.0, y: 0.0, u: 1.0, v: 0.0},
            Vertex2d { x: 160.0, y: 144.0, u: 1.0, v: 1.0},
        ],
                                                     None).unwrap();

        let mut pos = ivec2(0, 0);

        while !self.gl_handler.borrow().wind_should_close() {
            for _event in self.gl_handler.borrow_mut().handle_events() {

            }

            if self.gl_handler.borrow().get_window().has_resized_this_frame() { self.resize(); }

            self.framebuffer.bind_draw_target();

            match self.shader_manager.bind("UI".to_string()) {
                Ok(shader) => {
                    shader.set_uniform("screenPos".to_string(),pos);
                }
                Err(_) => return
            };
            pos.x += 1;
            if pos.x == 256 {
                pos.y += 1;
                pos.x = 0;
            }

            renderable.draw();

            FrameBuffer::bind_default_framebuffer();
            self.framebuffer.blit(
                self.gl_handler.borrow().get_window().get_window_size(),
                gl::COLOR_BUFFER_BIT,
                gl::NEAREST,
            );

            self.gl_handler.borrow_mut().poll_window();
        }
    }

    fn resize(&mut self) {
        let window_size = self.gl_handler.borrow().get_window().get_window_size();

        self.framebuffer.resize(window_size.x, window_size.y);
    }
}
