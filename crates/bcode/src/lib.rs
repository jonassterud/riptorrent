//! # Bcode
//!
//! `bcode` is a library for decoding and encoding to bcode.

// TODO:
// * Make sure dictionaries are sorted.
// * Write more test cases.

mod decode;
mod encode;
mod value;

pub use decode::decode;
pub use encode::encode;
pub use value::Value;
