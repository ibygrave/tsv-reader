#![no_std]

mod err;
mod prim;
mod reader;

pub use err::Error;
pub use reader::{Document, Read, Walker};
