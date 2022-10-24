use crate::*;
use anyhow::{anyhow, Result};
use std::collections::BTreeMap;

/// Decode data as bcode to a `bcode::Value`.
///
/// # Arguments
///
/// * `data` - bytes to decode.
/// * `index` - index of where to start decoding, usually `0`.
pub fn decode(data: &[u8], index: &mut usize) -> Result<Value> {
    match get(data, *index)? {
        // Integer
        b'i' => decode_integer(data, index),
        // Byte string
        48..=57 => decode_byte_string(data, index),
        // List
        b'l' => decode_list(data, index),
        // Dictionary
        b'd' => decode_dictionary(data, index),
        // Other
        _ => Err(anyhow!("Unexpected byte")),
    }
}

/// Helper function to get byte at index
///
/// # Arguments
///
/// * `data` - reference to data.
/// * `at` - index to get byte from.
fn get(data: &[u8], at: usize) -> Result<u8> {
    if at > data.len() - 1 {
        Err(anyhow!(
            "Index out of range (index: {}, data length: {})",
            at,
            data.len()
        ))
    } else {
        Ok(*data.get(at).unwrap())
    }
}

/// Decode a bencoded integer.
///
/// # Arguments
///
/// * `data` - bytes to decode.
/// * `index` - index of where to start decoding.
fn decode_integer(data: &[u8], index: &mut usize) -> Result<Value> {
    if get(data, *index)? as char == 'i' {
        *index += 1;
    } else {
        return Err(anyhow!("Integers must start with 'i'"));
    }

    let mut number_buf: Vec<char> = vec![];

    loop {
        match get(data, *index)? {
            // Minus sign
            b'-' => {
                if !number_buf.is_empty() {
                    return Err(anyhow!("Unexpected minus sign"));
                }

                number_buf.push(get(data, *index)? as char);
            }
            // Digits
            48..=57 => {
                number_buf.push(get(data, *index)? as char);

                if number_buf.first() == Some(&'-') && get(data, *index)? as char == '0' {
                    return Err(anyhow!("\"-0\" is not allowed"));
                }

                if number_buf.first() == Some(&'0') && number_buf.len() > 1 {
                    return Err(anyhow!("Leading zeros are not allowed"));
                }
            }
            // End character 'e'
            b'e' => {
                *index += 1;

                return Ok(Value::Integer(
                    number_buf.iter().collect::<String>().parse()?,
                ));
            }
            // Other
            _ => return Err(anyhow!("Unexpected byte while decoding integer")),
        }

        // Increase index
        *index += 1;
    }
}

/// Decode a bencoded byte string.
///
/// # Arguments
///
/// * `data` - bytes to decode.
/// * `index` - index of where to start decoding.
fn decode_byte_string(data: &[u8], index: &mut usize) -> Result<Value> {
    let mut length_buf: Vec<char> = vec![];

    loop {
        match get(data, *index)? {
            // Digits
            48..=57 => {
                length_buf.push(get(data, *index)? as char);

                if length_buf.first() == Some(&'0') && length_buf.len() > 1 {
                    return Err(anyhow!("Leading zeros are not allowed"));
                }
            }
            // Seperator ':'
            b':' => {
                let length = length_buf.iter().collect::<String>().parse::<usize>()?;
                let mut byte_string: Vec<u8> = vec![];

                for _ in 0..length {
                    *index += 1;
                    byte_string.push(get(data, *index)?);
                }

                *index += 1;

                return Ok(Value::ByteString(byte_string));
            }
            // Other
            _ => return Err(anyhow!("Unexpected byte while decoding byte string")),
        }

        // Increase index
        *index += 1;
    }
}

/// Decode a bencoded list.
///
/// # Arguments
///
/// * `data` - bytes to decode.
/// * `index` - index of where to start decoding.
fn decode_list(data: &[u8], index: &mut usize) -> Result<Value> {
    if get(data, *index)? as char == 'l' {
        *index += 1;
    } else {
        return Err(anyhow!("Lists must start with 'l'"));
    }

    let mut list = vec![];

    loop {
        match get(data, *index)? {
            // End character 'e'
            b'e' => {
                *index += 1;

                return Ok(Value::List(list));
            }
            // Other
            _ => list.push(decode(data, index)?),
        }
    }
}

/// Decode a bencoded dictionary.
///
/// # Arguments
///
/// * `data` - bytes to decode.
/// * `index` - index of where to start decoding.
fn decode_dictionary(data: &[u8], index: &mut usize) -> Result<Value> {
    if get(data, *index)? as char == 'd' {
        *index += 1;
    } else {
        return Err(anyhow!("Dictionaries must start with 'd'"));
    }

    let mut dictionary: BTreeMap<Vec<u8>, Value> = BTreeMap::new();
    let mut current_key: Option<Vec<u8>> = None;

    loop {
        match get(data, *index)? {
            // End character 'e'
            b'e' => {
                *index += 1;

                return Ok(Value::Dictionary(dictionary));
            }
            // Other
            _ => {
                if current_key.is_none() {
                    if let Ok(Value::ByteString(new_key)) = decode_byte_string(data, index) {
                        current_key = Some(new_key);
                    } else {
                        return Err(anyhow!("Dictionary key must be a byte string"));
                    }
                } else {
                    let new_value = decode(data, index)?;
                    dictionary.insert(current_key.unwrap(), new_value);
                    current_key = None;
                }
            }
        }
    }
}
