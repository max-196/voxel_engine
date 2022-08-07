use std::{
    error::Error,
    fmt::{self, Debug, Display},
    path::{Path, PathBuf},
    io::Read,
};

pub fn read_file(path: &Path) -> Result<(String, usize), FileError> {
    let mut contents = String::new();
    match std::fs::File::open(path) {
        Ok(mut v) => match v.read_to_string(&mut contents) {
            Ok(v) => Ok((contents, v)),
            Err(e) => Err(FileReadIOError(e, path.to_owned())),
        }
        Err(e) => Err(FileReadIOError(e, path.to_owned())),
    }
}

pub fn read_texture(path: &Path) -> Result<(Vec<u8>, png::OutputInfo), FileError> {
    let decoder = png::Decoder::new(
        match std::fs::File::open(path) {
            Ok(v) => v,
            Err(e) => return Err(TextureReadIOError(e, path.to_owned()))
        }
    );
    let mut reader = match decoder.read_info() {
        Ok(v) => v,
        Err(e) => return Err(TextureReadDecodingError(e, path.to_owned()))
    };
    let mut img_data = vec![0; reader.output_buffer_size()];
    let info = match reader.next_frame(&mut img_data) {
        Ok(v) => v,
        Err(e) => return Err(TextureReadDecodingError(e, path.to_owned()))
    };

    Ok((img_data, info))
}

// ERROR HANDLING

use png::DecodingError;
pub enum FileError {
    FileReadIOError(std::io::Error, PathBuf),
    TextureReadIOError(std::io::Error, PathBuf),
    TextureReadDecodingError(DecodingError, PathBuf),
}

use FileError::*;
impl Display for FileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FileReadIOError(e, p) => write!(f, "Couldn't read text-based file at path '{}': {}", p.display(), e),
            TextureReadIOError(e, p) => write!(f, "Couldn't read texture at path '{}': {}", p.display(), e),
            TextureReadDecodingError(e, p) => write!(f, "Couldn't decode texture at path '{}': {}", p.display(), e),
        }
    }
}
impl Debug for FileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(self, f)
    }
}
impl Error for FileError {}