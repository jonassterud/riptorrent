use super::Value;
use anyhow::Result;
use std::collections::BTreeMap;

/// Figures out which parse function to use for encoding
///
/// # Arguments
///
/// * `data` - data to encode
/// * `index` - mutable reference to the (data) index
pub fn any(data: &Value) -> Result<Vec<u8>> {
    match data {
        // Integer
        Value::Integer(inner) => integer(inner),
        // Byte string
        Value::ByteString(inner) => byte_string(inner),
        // List
        Value::List(inner) => list(inner),
        // Dictionary
        Value::Dictionary(inner) => dictionary(inner),
    }
}

/// Encodes integers
///
/// # Arguments
///
/// * `data` - i64 to encode
pub fn integer(data: &i64) -> Result<Vec<u8>> {
    let mut out: Vec<u8> = vec![];

    out.push(b'i');
    out.append(&mut data.to_string().as_bytes().to_vec());
    out.push(b'e');

    Ok(out)
}

/// Encodes byte strings
///
/// # Arguments
///
/// * `data` - Vec<u8> to encode
pub fn byte_string(data: &Vec<u8>) -> Result<Vec<u8>> {
    let mut out: Vec<u8> = vec![];

    out.append(&mut data.len().to_string().as_bytes().to_vec());
    out.push(b':');
    out.append(&mut data.clone());

    Ok(out)
}

/// Encodes list
///
/// # Arguments
///
/// * `data` - Vec<Value> to encode
pub fn list(data: &Vec<Value>) -> Result<Vec<u8>> {
    let mut out: Vec<u8> = vec![];

    out.push(b'l');

    for value in data {
        out.append(&mut any(value)?);
    }

    out.push(b'e');

    Ok(out)
}

/// Encodes dictionaries
///
/// # Arguments
///
/// * `data` - BTreeMap<Vec<u8>, Value> to encode
pub fn dictionary(data: &BTreeMap<Vec<u8>, Value>) -> Result<Vec<u8>> {
    let mut out: Vec<u8> = vec![];

    out.push(b'd');

    for (k, v) in data {
        out.append(&mut byte_string(k)?);
        out.append(&mut any(v)?);
    }

    out.push(b'e');

    Ok(out)
}
