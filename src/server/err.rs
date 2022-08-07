use std::{error::Error, fmt::{self, Debug, Display}};

pub enum ServerInitError {
    SocketError(std::io::Error)
}

use ServerInitError::*;

impl Display for ServerInitError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SocketError(e) => write!(f, "Error initializing server: Error initializing socket: {}", e),
        }
    }
}

impl Debug for ServerInitError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(self, f)
    }
}

impl Error for ServerInitError {}