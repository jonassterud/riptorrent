use crate::*;
use std::collections::BTreeMap;
use anyhow::Result;

/// Encode a `bcode::Value` to bcode.
///
/// # Arguments
///
/// * `data` - a `bcode::Value` to decode.
pub fn encode(data: Value) -> Result<Vec<u8>> {
    match data {
        Value::Integer(inner) => encode_integer(inner),
        Value::ByteString(inner) => encode_byte_string(inner),
        Value::List(inner) => encode_list(inner),
        Value::Dictionary(inner) => encode_dictionary(inner),
    }
}

/// Encode an integer to bencode.
///
/// # Arguments
///
/// * `data` - integer to encode.
fn encode_integer(data: i64) -> Result<Vec<u8>> {
    let mut out: Vec<u8> = vec![];

    out.push(b'i');
    out.append(&mut data.to_string().as_bytes().to_vec());
    out.push(b'e');

    Ok(out)
}

/// Encode a byte string to bencode.
///
/// # Arguments
///
/// * `data` - `Vec<u8>` to encode.
pub fn encode_byte_string(data: Vec<u8>) -> Result<Vec<u8>> {
    let mut out: Vec<u8> = vec![];

    out.append(&mut data.len().to_string().as_bytes().to_vec());
    out.push(b':');

    let mut temp = data;
    out.append(&mut temp);

    Ok(out)
}

/// Encode a list to bencode.
///
/// # Arguments
///
/// * `data` - `Vec<Value>` to encode.
pub fn encode_list(data: Vec<Value>) -> Result<Vec<u8>> {
    let mut out: Vec<u8> = vec![];

    out.push(b'l');

    for value in data {
        out.append(&mut encode(value)?);
    }

    out.push(b'e');

    Ok(out)
}

/// Encode a dictionary to bencode.
///
/// # Arguments
///
/// * `data` - `BTreeMap<Vec<u8>, Value>` to encode
pub fn encode_dictionary(data: BTreeMap<Vec<u8>, Value>) -> Result<Vec<u8>> {
    let mut out: Vec<u8> = vec![];
    
    out.push(b'd');

    for (k, v) in data {
        out.append(&mut encode_byte_string(k)?);
        out.append(&mut encode(v)?);
    }

    out.push(b'e');

    Ok(out)
}