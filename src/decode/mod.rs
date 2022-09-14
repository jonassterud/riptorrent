pub mod parse;

use anyhow::{anyhow, Result};
use std::collections::BTreeMap;

#[derive(Debug, PartialEq)]
pub enum Value {
    Integer(i64),
    ByteString(Vec<u8>),
    List(Vec<Value>),
    Dictionary(BTreeMap<Vec<u8>, Value>),
}

/// Decodes bencoded data
pub fn decode(data: &mut [u8], index: &mut usize) -> Result<Vec<Value>> {
    let mut out = vec![];

    while *index + 1 < data.len() {
        out.push(parse::any(data, index)?);
    }

    Ok(out)
}
