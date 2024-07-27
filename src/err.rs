//! Error types

use core::{fmt::Display, num::ParseIntError, str::Utf8Error};

/// Error type of parsers.
#[derive(PartialEq, Debug)]
pub enum Error {
    /// Document data is not valid UTF8.
    Utf8,
    /// Error when parsing a field.
    ParseField,
    /// Unexpectedly reached end of document.
    EndOfDocument,
    /// Unexpectedly reached end of line.
    EndOfLine,
    /// Line contained surplus unparsed fields.
    SurplusFields,
}

impl Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Error::Utf8 => write!(f, "Error::Utf8"),
            Error::ParseField => write!(f, "Error::ParseField"),
            Error::EndOfDocument => write!(f, "Error::EndOfDocument"),
            Error::EndOfLine => write!(f, "Error::EndOfLine"),
            Error::SurplusFields => write!(f, "Error::SurplusFields"),
        }
    }
}

impl From<Utf8Error> for Error {
    fn from(_value: Utf8Error) -> Self {
        Error::Utf8
    }
}

impl From<ParseIntError> for Error {
    fn from(_value: ParseIntError) -> Self {
        Error::ParseField
    }
}
