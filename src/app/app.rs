use std::cell::RefCell;
use std::rc::Rc;
use dec_gl::{FrameBuffer, GLHandler, Renderable, UICamera, Vertex2d};
use dec_gl::shader::{ShaderManager, ShaderProgram};
use dec_gl::texture::{Texture2D, TextureManager};
use glm::ivec2;

pub struct App {
    pub args: Vec<String>,

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
            args,

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
            }
            Err(_) => {}
        }
        self.texture_manager.bind("test".to_string());

        let renderable = Renderable::new_initialised(&vec![
            Vertex2d { x: 10.0, y: 10.0, u: 0.0, v: 0.0},
            Vertex2d { x: 10.0, y: 50.0, u: 0.0, v: 1.0},
            Vertex2d { x: 50.0, y: 10.0, u: 1.0, v: 0.0},

            Vertex2d { x: 10.0, y: 50.0, u: 0.0, v: 1.0},
            Vertex2d { x: 50.0, y: 10.0, u: 1.0, v: 0.0},
            Vertex2d { x: 50.0, y: 50.0, u: 1.0, v: 1.0},
        ],
                                                     None).unwrap();

        while !self.gl_handler.borrow().wind_should_close() {
            for _event in self.gl_handler.borrow_mut().handle_events() {

            }

            renderable.draw();

            self.gl_handler.borrow_mut().poll_window();
        }
    }
}
