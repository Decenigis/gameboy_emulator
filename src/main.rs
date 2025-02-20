mod app;
mod memory;
mod renderer;
mod cpu;

use std::env;
use dec_gl::GLHandler;
use crate::app::App;

fn main() {
    match GLHandler::new("GB Emulator",
                         800,
                         720,
                         false,
                         true)
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
