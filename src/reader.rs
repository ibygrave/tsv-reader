#[cfg(feature = "derive")]
pub use tsv_reader_derive::*;

use crate::err::Error;

pub struct Walker<'a>(core::str::Split<'a, char>);

impl<'a> Walker<'a> {
    pub fn next_field(&mut self) -> Result<&'a str, Error> {
        self.0.next().ok_or(Error)
    }
    pub fn parse_one<T: Read<'a>>(&mut self) -> Result<T, Error> {
        T::parse_tsv(self)
    }
    fn parse_one_line<T: Read<'a>>(line: &'a str) -> Result<T, Error> {
        Self(line.split('\t')).parse_one()
    }
}

pub trait Read<'a>: Sized {
    fn parse_tsv(fields: &mut Walker<'a>) -> Result<Self, Error>;
}

pub struct Document<'a>(core::str::Split<'a, char>);

impl<'a> Document<'a> {
    pub fn new(data: &'a [u8]) -> Result<Self, Error> {
        Ok(Self(core::str::from_utf8(data)?.split('\n')))
    }
    pub fn parse_one<T: Read<'a>>(&mut self) -> Result<T, Error> {
        Walker::parse_one_line(self.0.next().ok_or(Error)?)
    }
    pub fn parse_iter<T: Read<'a>>(self) -> impl Iterator<Item = T> + 'a {
        self.0.map_while(|line| Walker::parse_one_line(line).ok())
    }
}
