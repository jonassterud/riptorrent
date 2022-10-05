pub mod parse;

use super::Value;

use anyhow::Result;

/// Encodes data to bencode
///
/// # Arguments
///
/// * `data` - data to encode
pub fn encode(data: Vec<&Value>) -> Result<Vec<u8>> {
    let mut out = vec![];

    for value in data {
        out.append(&mut parse::any(value)?);
    }

    Ok(out)
}
