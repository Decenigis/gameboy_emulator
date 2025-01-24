mod app;

use std::env;
use dec_gl::{GLHandler, Renderable, UICamera, Vertex2d};
use crate::app::App;

fn main() {
    match GLHandler::new("bob",
                         800,
                         720,
                         false,
                         false)
    {
        Ok(gl_handler) => {
            let mut app = App::new(env::args().collect(), gl_handler.clone());
            app.run();
        },
        Err(_e) => {
            std::process::exit(exitcode::UNAVAILABLE);
        }
    };
}
