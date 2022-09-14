use super::Value;
use anyhow::{anyhow, Result};
use std::collections::BTreeMap;

/// Figures out what is encoded and calls the correct parse function
pub fn any(data: &mut [u8], index: &mut usize) -> Result<Value> {
    match data
        .get(*index)
        .ok_or_else(|| anyhow!("Index out of range"))?
    {
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
pub fn integer(data: &mut [u8], index: &mut usize) -> Result<Value> {
    if *data
        .get(*index)
        .ok_or_else(|| anyhow!("Index out of range"))? as char
        == 'i'
    {
        *index += 1;
    } else {
        return Err(anyhow!("Integers must start with 'i'"));
    }

    let mut number_buf: Vec<char> = vec![];

    loop {
        match data
            .get(*index)
            .ok_or_else(|| anyhow!("Index out of range"))?
        {
            // Minus sign
            45 => {
                if !number_buf.is_empty() {
                    return Err(anyhow!("Unexpected byte while parsing integer"));
                }

                number_buf.push(*data.get(*index).unwrap() as char);
            }
            // Digits
            48..=57 => {
                number_buf.push(*data.get(*index).unwrap() as char);

                if number_buf.first() == Some(&'-') && data.get(*index).unwrap() == &48 {
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
pub fn byte_string(data: &mut [u8], index: &mut usize) -> Result<Value> {
    let mut length_buf: Vec<char> = vec![];

    loop {
        match data
            .get(*index)
            .ok_or_else(|| anyhow!("Index out of range"))?
        {
            // Digits
            48..=57 => {
                length_buf.push(*data.get(*index).unwrap() as char);

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
                    byte_string.push(*data.get(*index).unwrap());
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
pub fn list(data: &mut [u8], index: &mut usize) -> Result<Value> {
    if *data
        .get(*index)
        .ok_or_else(|| anyhow!("Index out of range"))? as char
        == 'l'
    {
        *index += 1;
    } else {
        return Err(anyhow!("Lists must start with 'l'"));
    }

    let mut list = vec![];

    loop {
        match data
            .get(*index)
            .ok_or_else(|| anyhow!("Index out of range"))?
        {
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
pub fn dictionary(data: &mut [u8], index: &mut usize) -> Result<Value> {
    if *data
        .get(*index)
        .ok_or_else(|| anyhow!("Index out of range"))? as char
        == 'd'
    {
        *index += 1;
    } else {
        return Err(anyhow!("Dictionaries must start with 'd'"));
    }

    let mut dictionary: BTreeMap<Vec<u8>, Value> = BTreeMap::new();
    let mut current_key: Option<Vec<u8>> = None;

    loop {
        match data
            .get(*index)
            .ok_or_else(|| anyhow!("Index out of range"))?
        {
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
