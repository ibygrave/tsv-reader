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
//! const DOC: &[u8] = br#"1	Example Title	FFFFFF
//! 000000	false	Line	0	0	500	500
//! 550055	true	Circle	200	300	20
//! FF0055	false	Rectangle	100	100	200	200
//! "#;
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
