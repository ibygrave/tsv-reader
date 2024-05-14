//! The `Reader` trait.

#[cfg(feature = "derive")]
pub use tsv_reader_derive::*;

use crate::err::Error;

/// An iterator over fields of a single line,
/// as parsed into a single data value.
pub struct Walker<'doc>(core::str::Split<'doc, char>);

impl<'doc> Walker<'doc> {
    /// Return the next unparsed field.
    /// Returns an error if there are no more fields to parse
    /// on this line.
    pub fn next_field(&mut self) -> Result<&'doc str, Error> {
        self.0.next().ok_or(Error::EndOfLine)
    }

    /// Parse a value of type `T` from the remaining fields
    /// on this line.
    /// Called recursively to parse nested data structures.
    /// Returns an error if the end of line is unexpectedly reached,
    /// or is a field is not of the correct format.
    pub fn parse_one<T: Read<'doc>>(&mut self) -> Result<T, Error> {
        T::parse_tsv(self)
    }

    fn parse_one_line<T: Read<'doc>>(line: &'doc str) -> Result<T, Error> {
        Self(line.split('\t')).parse_one()
    }
}

/// The trait that must be implemented by types which can be
/// constructed from a sequence of text fields.
///
/// If the `derive` feature is enabled then this trait
/// can be automatically derived for structs and enums
/// consisting only of other types which implement `Read`.
pub trait Read<'doc>: Sized {
    /// Retrieve 0 or more fields from `fields` and convert
    /// them into a value of type `Self`,
    /// or return an error if parsing fails.
    fn parse_tsv(fields: &mut Walker<'doc>) -> Result<Self, Error>;
}

/// An iterator over the lines of a TSV document.
pub struct Document<'doc>(core::str::Split<'doc, char>);

impl<'doc> Document<'doc> {
    /// Initialise a `Document` from a byte sequence.
    /// Returns an error if the data is not valid utf-8.
    pub fn new(data: &'doc [u8]) -> Result<Self, Error> {
        Ok(Self(core::str::from_utf8(data)?.split('\n')))
    }

    /// Parses the next line of the document as a value of type `T`.
    /// Returns an error if parsing fails.
    ///
    /// Note: If parsing completes without using all the fields
    /// on the line, this is not an error. Unused fields are ignored.
    ///
    /// TODO: Make surplus fields into an error?
    pub fn parse_one<T: Read<'doc>>(&mut self) -> Result<T, Error> {
        Walker::parse_one_line(self.0.next().ok_or(Error::EndOfDocument)?)
    }

    /// Parses all the remaining lines of the document as values of type `T`,
    /// iterating over the parsed values.
    /// This method consumes the document.
    ///
    /// Note: Parsing errors terminate the iterator but do not return an error.
    ///
    /// TODO: Iterate over `Result<T>` instead of dropping parsing errors.
    pub fn parse_iter<T: Read<'doc>>(self) -> impl Iterator<Item = T> + 'doc {
        self.0.map_while(|line| Walker::parse_one_line(line).ok())
    }
}
