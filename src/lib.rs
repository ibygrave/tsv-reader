#![no_std]

//! TODO: Describe the purpose and use of this crate.
//!
//! Minimal parser for tab-seperated-value (TSV) files.
//! Usable in `no_std` and without `alloc`.
//! The supported file format differes from standard TSV
//! by omitting the header line, and allowing each line
//! to encode a different type.
//!
//! TODO: Move examples into documentation.

mod err;
mod prim;
mod reader;

pub use err::Error;
pub use reader::{Document, Read, Walker};
