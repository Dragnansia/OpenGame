use std::{env::VarError, fmt::Display, io};

#[derive(Debug)]
pub struct Error(String);

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Self(err.to_string())
    }
}

impl From<VarError> for Error {
    fn from(err: VarError) -> Self {
        Self(err.to_string())
    }
}

impl From<String> for Error {
    fn from(err: String) -> Self {
        Self(err)
    }
}

impl From<&str> for Error {
    fn from(err: &str) -> Self {
        Self(err.into())
    }
}
