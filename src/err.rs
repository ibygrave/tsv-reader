//! Error types

use core::{fmt::Display, num::ParseIntError, str::Utf8Error};

/// Error type of parsers.
/// TODO: Provide details of which error happened.
#[derive(Debug)]
pub struct Error;

impl Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "TsvError")
    }
}

#[cfg(std)]
impl std::error::Error for Error {}

impl From<Utf8Error> for Error {
    fn from(_value: Utf8Error) -> Self {
        Error
    }
}

impl From<ParseIntError> for Error {
    fn from(_value: ParseIntError) -> Self {
        Error
    }
}
