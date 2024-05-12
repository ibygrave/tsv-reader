#![no_std]

//! Minimal parser for tab-seperated-value (TSV) files.
//! Usable in `no_std` and without `alloc`.
//! The supported file format differes from standard TSV
//! by omitting the header line, and allowing each line
//! to encode a different type.
//!
//! ## Example
//! ```rust
//! # use tsv_reader::*;
//! // A TSV file where the first line is a `Header` and the rest are `Object`s.
//! const DATA: &[u8] = br#"1	Example Title	FFFFFF
//! 000000	false	Line	0	0	500	500
//! 550055	true	Circle	200	300	20
//! FF0055	false	Rectangle	100	100	200	200
//! "#;
//!
//! #[derive(PartialEq, Debug, Read)]
//! struct Colour([u8; 3]);
//!
//! #[derive(PartialEq, Debug, Read)]
//! struct Header<'a> {
//!     version: u32,
//!     title: &'a str,
//!     background: Colour,
//! }
//!
//! #[derive(PartialEq, Debug, Read)]
//! enum Shape {
//!     Line(u32, u32, u32, u32),
//!     Circle(u32, u32, u32),
//!     Rectange(u32, u32, u32, u32),
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
//!     let mut doc = Document::new(DATA).unwrap();
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
//!     println!("{:?}", header);
//!     println!("{:?}", objects);
//! }
//! ```

mod err;
mod prim;
mod reader;

pub use err::Error;
pub use reader::{Document, Read, Walker};
