use std::{error::Error, fmt::{self, Debug, Display}};

pub enum NetInitError {
    SocketError(std::io::Error)
}

use NetInitError::*;

impl Display for NetInitError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SocketError(e) => write!(f, "Error initializing socket: {}", e),
        }
    }
}

impl Debug for NetInitError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(self, f)
    }
}

impl Error for NetInitError {}