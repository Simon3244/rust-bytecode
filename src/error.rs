use std::{io, result, string::FromUtf8Error};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("IO error: {0}")]
    IO(io::Error),

    #[error("Invalid magic number: {0:#X}")]
    InvalidMagicNumber(u32),

    #[error("Unrecognized {0}: {1}")]
    Unrecognized(&'static str, String),

    #[error("FromUtf8 error: {0}")]
    Utf8Error(FromUtf8Error),

    #[error("Invalid access flags: {0:#X}")]
    InvalidAccessFlags(u16),

    #[error("{0}")]
    Other(String),
}

impl From<io::Error> for ParseError {
    fn from(err: io::Error) -> Self {
        ParseError::IO(err)
    }
}

impl From<FromUtf8Error> for ParseError {
    fn from(err: FromUtf8Error) -> Self {
        ParseError::Utf8Error(err)
    }
}

pub type Result<T> = result::Result<T, ParseError>;
