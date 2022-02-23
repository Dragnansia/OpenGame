use log::SetLoggerError;

use crate::error::dir;
use std::{env::VarError, fmt::Display, io};

#[derive(Debug)]
pub struct Error(String);

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<dir::Error> for Error {
    fn from(err: dir::Error) -> Self {
        Self(err.to_string())
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

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Self(err.to_string())
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Self(err.to_string())
    }
}

impl From<SetLoggerError> for Error {
    fn from(err: SetLoggerError) -> Self {
        Self(err.to_string())
    }
}

impl From<purs::error::Error> for Error {
    fn from(error: purs::error::Error) -> Self {
        Self(error.to_string())
    }
}
