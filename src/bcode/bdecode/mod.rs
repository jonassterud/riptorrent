pub mod parse;

use super::Value;

use anyhow::Result;

/// Decodes bencoded data
///
/// # Arguments
///
/// * `data` - mutable reference to data to decode
/// * `index` - mutable reference to the (data) index
pub fn decode(data: &mut Vec<u8>, index: &mut usize) -> Result<Vec<Value>> {
    let mut out = vec![];

    while *index + 1 < data.len() {
        out.push(parse::any(data, index)?);
    }

    Ok(out)
}
