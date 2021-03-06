//! Basic error Implementation

use std::io::ErrorKind;

pub enum Error {
    /// Give command with error
    CommandError(String),
    /// Return data name not found
    DataNotFound(String),
    /// Return the file path not found
    FileNotFound(String),
    /// Return the path not found
    PathNotFound(String),
    Custom(String),
    Unknown,
}

impl Default for Error {
    fn default() -> Self {
        Self::Unknown
    }
}

impl From<std::io::Error> for Error {
    fn from(io_err: std::io::Error) -> Self {
        match io_err.kind() {
            ErrorKind::NotFound => Self::FileNotFound(format!("{:?}", io_err.raw_os_error())),
            _ => Self::Unknown,
        }
    }
}
