#![no_std]

//! Minimal parser for tab-seperated-value (TSV) data.
//! Usable in `no_std` and without `alloc`.
//! The supported data format differs from standard TSV
//! by omitting the header line, and allowing each line
//! to encode a different type.
//!
//! ## Example
//! ```rust
//! # use tsv_reader::*;
//! // A TSV document where the first line is a `Header` and the rest are `Object`s.
//! const DOC: &[u8] = b"1\tExample Title\tFFFFFF
//! 000000\tfalse\tLine\t0\t0\t500\t500
//! 550055\ttrue\tCircle\t200\t300\t20
//! FF0055\tfalse\tRectangle\t100\t100\t200\t200";
//!
//! #[derive(PartialEq, Debug, Read)]
//! struct Colour([u8; 3]);
//!
//! #[derive(PartialEq, Debug, Read)]
//! struct Header<'doc> {
//!     version: u32,
//!     title: &'doc str,
//!     background: Colour,
//! }
//!
//! #[derive(PartialEq, Debug, Read)]
//! enum Shape {
//!     Line(u32, u32, u32, u32),
//!     Circle(u32, u32, u32),
//!     Rectangle(u32, u32, u32, u32),
//! }
//!
//! #[derive(PartialEq, Debug, Read)]
//! struct Object {
//!     colour: Colour,
//!     fill: bool,
//!     shape: Shape,
//! }
//!
//! fn main() {
//!     let mut doc = Document::new(DOC).unwrap();
//!     let header: Header = doc.parse_one().unwrap();
//!     let objects: Vec<Object> = doc.parse_iter().collect();
//!
//!     assert_eq!(
//!         header,
//!         Header {
//!             version: 1,
//!             title: "Example Title",
//!             background: Colour([255, 255, 255])
//!         }
//!     );
//!     assert_eq!(objects.len(), 3);
//!     assert_eq!(
//!         objects[1],
//!         Object {
//!             colour: Colour([0x55, 0x00, 0x55]),
//!             fill: true,
//!             shape: Shape::Circle(200, 300, 20)
//!         }
//!     );
//! }
//! ```

mod err;
mod prim;
mod reader;

pub use err::Error;
pub use reader::{Document, Read, Walker};

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(PartialEq, Debug, Read)]
    struct Pair(u8, u8);

    #[test]
    fn test_bad_utf8() {
        let doc = Document::new(b"\xF0\xA4\xAD");
        assert!(doc.is_err());
        assert_eq!(doc.err().unwrap(), Error::Utf8);
    }

    #[test]
    fn test_parse_field() {
        let mut doc = Document::new(b"abc").unwrap();
        let obj: Result<u8, Error> = doc.parse_one();
        assert!(obj.is_err());
        assert_eq!(obj.err().unwrap(), Error::ParseField);
    }

    #[test]
    fn test_parse_subfield() {
        let mut doc = Document::new(b"42\txyz").unwrap();
        let obj: Result<Pair, Error> = doc.parse_one();
        assert!(obj.is_err());
        assert_eq!(obj.err().unwrap(), Error::ParseField);
    }

    #[test]
    fn test_endofdocument() {
        let mut doc = Document::new(b"42\t31").unwrap();
        let _: Pair = doc.parse_one().unwrap();
        let obj2: Result<Pair, Error> = doc.parse_one();
        assert!(obj2.is_err());
        assert_eq!(obj2.err().unwrap(), Error::EndOfDocument);
    }

    #[test]
    fn test_endofline() {
        let mut doc = Document::new(b"42").unwrap();
        let obj: Result<Pair, Error> = doc.parse_one();
        assert!(obj.is_err());
        assert_eq!(obj.err().unwrap(), Error::EndOfLine);
    }

    #[test]
    fn test_surplusfields() {
        let mut doc = Document::new(b"42\t21\t0").unwrap();
        let obj: Result<Pair, Error> = doc.parse_one();
        assert!(obj.is_err());
        assert_eq!(obj.err().unwrap(), Error::SurplusFields);
    }
}
