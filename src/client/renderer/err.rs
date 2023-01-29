use {
    std::{error::Error, fmt::{self, Debug, Display}},
    crate::common::files::FileError
};

pub enum RendererError {
    ShaderCreation(FileError),
    TextureCreation(FileError),
    SurfaceCreation(wgpu::CreateSurfaceError),
    AdapterNotFound,
    RequestDevice(wgpu::RequestDeviceError),
}

use RendererError::*;

impl Display for RendererError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ShaderCreation(e) => write!(f, "Error creating shader: {e}"),
            TextureCreation(e) => write!(f, "Texture error: {e}"),
            SurfaceCreation(e) => write!(f, "Error creating surface: {e}"),
            AdapterNotFound => write!(f, "Adapter couldn't be found"),
            RequestDevice(e) => write!(f, "Error requesting device: {e}"),
        }
    }
}

impl Debug for RendererError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(self, f)
    }
}

impl Error for RendererError {}