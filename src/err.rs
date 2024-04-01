use std::{fmt::Display, num::ParseIntError, str::Utf8Error};

#[derive(Debug)]
pub struct Error;

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TsvError")
    }
}

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
