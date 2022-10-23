//! # Bcode
//!
//! `bcode` is a library for decoding and encoding to bcode.

// TODO:
// * Make sure dictionaries are sorted.
// * Write more test cases.

mod decode;
mod encode;
mod value;

pub use crate::decode::decode;
pub use crate::encode::encode;
pub use crate::value::{Value, map_get};
