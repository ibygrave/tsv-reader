#[cfg(feature = "derive")]
pub use tsv_reader_derive::*;

use crate::err::Error;

pub struct Walker<'a>(std::str::Split<'a, char>);

impl<'a> Walker<'a> {
    pub fn next_field(&mut self) -> Result<&'a str, Error> {
        self.0.next().ok_or(Error)
    }
    pub fn parse_one<T: Read<'a>>(&mut self) -> Result<T, Error> {
        T::parse_tsv(self)
    }
    pub fn parse_one_line<T: Read<'a>>(line: &'a str) -> Result<T, Error> {
        Self(line.split('\t')).parse_one()
    }
}

pub trait Read<'a>: Sized {
    fn parse_tsv(fields: &mut Walker<'a>) -> Result<Self, Error>;
}
