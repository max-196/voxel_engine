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
            Err(e) => Err(FileReadIO(e, path.to_owned())),
        }
        Err(e) => Err(FileReadIO(e, path.to_owned())),
    }
}

pub fn read_texture(path: &Path) -> Result<(Vec<u8>, png::OutputInfo), FileError> {
    let decoder = png::Decoder::new(
        match std::fs::File::open(path) {
            Ok(v) => v,
            Err(e) => return Err(TextureReadIO(e, path.to_owned()))
        }
    );
    let mut reader = match decoder.read_info() {
        Ok(v) => v,
        Err(e) => return Err(TextureReadDecoding(e, path.to_owned()))
    };
    let mut img_data = vec![0; reader.output_buffer_size()];
    let info = match reader.next_frame(&mut img_data) {
        Ok(v) => v,
        Err(e) => return Err(TextureReadDecoding(e, path.to_owned()))
    };

    Ok((img_data, info))
}

// ERROR HANDLING

use png::DecodingError;
pub enum FileError {
    FileReadIO(std::io::Error, PathBuf),
    TextureReadIO(std::io::Error, PathBuf),
    TextureReadDecoding(DecodingError, PathBuf),
}

use FileError::*;
impl Display for FileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FileReadIO(e, p) => write!(f, "Couldn't read text-based file at path '{}': {}", p.display(), e),
            TextureReadIO(e, p) => write!(f, "Couldn't read texture at path '{}': {}", p.display(), e),
            TextureReadDecoding(e, p) => write!(f, "Couldn't decode texture at path '{}': {}", p.display(), e),
        }
    }
}
impl Debug for FileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(self, f)
    }
}
impl Error for FileError {}