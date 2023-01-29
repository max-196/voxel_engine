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
    RendererInit(renderer::err::RendererError),
    NetInit(net::err::NetInitError),
}

use ClientInitError::*;

impl Display for ClientInitError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RendererInit(e) => write!(f, "Renderer Initialization Error: {e}"),
            NetInit(e) => write!(f, "Networking Initialization Error: {e}"),
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
        RendererInit(error)
    }
}

impl From<net::err::NetInitError> for ClientInitError {
    fn from(error: net::err::NetInitError) -> Self {
        NetInit(error)
    }
}