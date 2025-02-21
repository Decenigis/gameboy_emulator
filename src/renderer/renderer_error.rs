#![allow(dead_code)]
use dec_gl::RenderError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RendererError {
    #[error("OpenGL Error: {error}")]
    GLError { error: RenderError }
}
