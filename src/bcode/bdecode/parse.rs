use super::Value;
use anyhow::{anyhow, Result};
use std::collections::BTreeMap;

/// Helper function to get byte at index
///
/// # Arguments
///
/// * `data` - reference to data
/// * `at` - index to get byte from
fn get(data: &[u8], at: &usize) -> Result<u8> {
    Ok(*data.get(*at).ok_or_else(|| anyhow!("Index out of range"))?)
}

/// Figures out what is encoded and calls the correct parse function
///
/// # Arguments
///
/// * `data` - mutable reference to data to decode
/// * `index` - mutable reference to the (data) index
pub fn any(data: &mut [u8], index: &mut usize) -> Result<Value> {
    match get(data, index)? {
        // Integer
        105 => integer(data, index),
        // Byte string
        48..=57 => byte_string(data, index),
        // List
        108 => list(data, index),
        // Dictionary
        100 => dictionary(data, index),
        // Other
        _ => Err(anyhow!("Unexpected byte")),
    }
}

/// Parses integers (example: i32e)
///
/// # Arguments
///
/// * `data` - mutable reference to data to decode
/// * `index` - mutable reference to the (data) index
pub fn integer(data: &mut [u8], index: &mut usize) -> Result<Value> {
    if get(data, index)? as char == 'i' {
        *index += 1;
    } else {
        return Err(anyhow!("Integers must start with 'i'"));
    }

    let mut number_buf: Vec<char> = vec![];

    loop {
        match get(data, index)? {
            // Minus sign
            45 => {
                if !number_buf.is_empty() {
                    return Err(anyhow!("Unexpected byte while parsing integer"));
                }

                number_buf.push(get(data, index)? as char);
            }
            // Digits
            48..=57 => {
                number_buf.push(get(data, index)? as char);

                if number_buf.first() == Some(&'-') && get(data, index)? == 48 {
                    return Err(anyhow!("\"-0\" is not allowed"));
                }

                if number_buf.first() == Some(&'0') && number_buf.len() > 1 {
                    return Err(anyhow!("Leading zeros are not allowed"));
                }
            }
            // End character 'e'
            101 => {
                *index += 1;

                return Ok(Value::Integer(
                    number_buf.iter().collect::<String>().parse()?,
                ));
            }
            // Other
            _ => return Err(anyhow!("Unexpected byte while parsing integer")),
        }

        // Increase index
        *index += 1;
    }
}

/// Parses byte strings (example: 5:hello)
///
/// # Arguments
///
/// * `data` - mutable reference to data to decode
/// * `index` - mutable reference to the (data) index
pub fn byte_string(data: &mut [u8], index: &mut usize) -> Result<Value> {
    let mut length_buf: Vec<char> = vec![];

    loop {
        match get(data, index)? {
            // Digits
            48..=57 => {
                length_buf.push(get(data, index)? as char);

                if length_buf.first() == Some(&'0') && length_buf.len() > 1 {
                    return Err(anyhow!("Leading zeros are not allowed"));
                }
            }
            // Seperator ':'
            58 => {
                let length: usize = length_buf.iter().collect::<String>().parse()?;
                let mut byte_string: Vec<u8> = vec![];

                for _ in 0..length {
                    *index += 1;
                    byte_string.push(get(data, index)?);
                }

                *index += 1;

                return Ok(Value::ByteString(byte_string));
            }
            // Other
            _ => return Err(anyhow!("Unexpected byte while parsing byte string")),
        }

        // Increase index
        *index += 1;
    }
}

/// Parses lists (example: li32e4:teste)
///
/// # Arguments
///
/// * `data` - mutable reference to data to decode
/// * `index` - mutable reference to the (data) index
pub fn list(data: &mut [u8], index: &mut usize) -> Result<Value> {
    if get(data, index)? as char == 'l' {
        *index += 1;
    } else {
        return Err(anyhow!("Lists must start with 'l'"));
    }

    let mut list = vec![];

    loop {
        match get(data, index)? {
            // End character 'e'
            101 => {
                *index += 1;

                return Ok(Value::List(list));
            }
            // Other
            _ => list.push(any(data, index)?),
        }
    }
}

/// Parses dictionaries (example: d3:key5:valuee)
///
/// # Arguments
///
/// * `data` - mutable reference to data to decode
/// * `index` - mutable reference to the (data) index
pub fn dictionary(data: &mut [u8], index: &mut usize) -> Result<Value> {
    if get(data, index)? as char == 'd' {
        *index += 1;
    } else {
        return Err(anyhow!("Dictionaries must start with 'd'"));
    }

    let mut dictionary: BTreeMap<Vec<u8>, Value> = BTreeMap::new();
    let mut current_key: Option<Vec<u8>> = None;

    loop {
        match get(data, index)? {
            // End character 'e'
            101 => {
                *index += 1;

                return Ok(Value::Dictionary(dictionary));
            }
            // Other
            _ => {
                if current_key.is_none() {
                    if let Ok(Value::ByteString(new_key)) = byte_string(data, index) {
                        current_key = Some(new_key);
                    } else {
                        return Err(anyhow!("Dictionary key must be a byte string"));
                    }
                } else {
                    let new_value = any(data, index)?;
                    dictionary.insert(current_key.unwrap(), new_value);
                    current_key = None;
                }
            }
        }
    }
}
