use {
    std::{error::Error, fmt::{self, Debug, Display}},
    crate::common::files::FileError
};

pub enum RendererError {
    ShaderCreationError(FileError),
    TextureCreationError(FileError),
    SurfaceCreationError(wgpu::CreateSurfaceError),
    AdapterNotFound,
    RequestDeviceError(wgpu::RequestDeviceError),
}

use RendererError::*;

impl Display for RendererError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ShaderCreationError(e) => write!(f, "Error creating shader: {}", e),
            TextureCreationError(e) => write!(f, "Texture error: {}", e),
            SurfaceCreationError(e) => write!(f, "Error creating surface: {}", e),
            AdapterNotFound => write!(f, "Adapter couldn't be found"),
            RequestDeviceError(e) => write!(f, "Error requesting device: {}", e),
        }
    }
}

impl Debug for RendererError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(self, f)
    }
}

impl Error for RendererError {}