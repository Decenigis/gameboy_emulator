#![allow(dead_code)]
use thiserror::Error;
use crate::renderer::RendererError;

#[derive(Error, Debug)]
pub enum SystemError {
    #[error("Renderer Error: {error}")]
    RendererError { error: RendererError }
}
