use {
    std::{
        error::Error,
        fmt::{self, Debug, Display}
    },
    super::{
        renderer,
        net,
    }
};

pub enum ClientInitError {
    RendererInitError(renderer::err::RendererError),
    NetInitError(net::error::NetInitError),
}

use ClientInitError::*;

impl Display for ClientInitError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RendererInitError(e) => write!(f, "Renderer Initialization Error: {}", e),
            NetInitError(e) => write!(f, "Networking Initialization Error: {}", e),
        }
    }
}

impl Debug for ClientInitError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(self, f)
    }
}

impl Error for ClientInitError {}

impl From<renderer::err::RendererError> for ClientInitError {
    fn from(error: renderer::err::RendererError) -> Self {
        RendererInitError(error)
    }
}

impl From<net::error::NetInitError> for ClientInitError {
    fn from(error: net::error::NetInitError) -> Self {
        NetInitError(error)
    }
}