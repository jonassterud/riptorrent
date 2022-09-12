use anyhow::{anyhow, Result};
use std::collections::BTreeMap;

#[derive(Debug, PartialEq)]
pub enum Value {
    Integer(i32),
    ByteString(Vec<u8>),
    List(Vec<Value>),
    Dictionary(BTreeMap<Vec<u8>, Value>),
}

pub fn decode(mut data: Vec<u8>) -> Result<Vec<Value>> {
    let mut vec: Vec<Value> = vec![];
    let mut index: usize = 0;

    while index + 1 < data.len() {
        vec.push(retrieve_and_decode_value(&mut data, &mut index)?);
    }

    Ok(vec)
}

fn retrieve_and_decode_value(data: &mut Vec<u8>, index: &mut usize) -> Result<Value> {
    match data.get(*index).unwrap() {
        // Integer
        105 => {
            let mut number_buf: Vec<char> = vec![];
            loop {
                *index += 1;
                
                match data.get(*index).unwrap() {
                    // Minus sign
                    45 => {
                        if !number_buf.is_empty() {
                            return Err(anyhow!("Unexpected byte"));
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
                    _ => return Err(anyhow!("Unexpected byte")),
                }
            }
        }
        // Byte string
        48..=57 => {
            let mut length_buf = (*data.get(*index).unwrap() as char).to_string();

            loop {
                *index += 1;

                match data.get(*index).unwrap() {
                    // Digits
                    48..=57 => length_buf.push(*data.get(*index).unwrap() as char),
                    // Seperator ':'
                    58 => {
                        let length: usize = length_buf.parse()?;
                        let mut byte_string: Vec<u8> = vec![];

                        for _ in 0..length {
                            *index += 1;
                            byte_string.push(*data.get(*index).unwrap());
                        }

                        *index += 1;

                        return Ok(Value::ByteString(byte_string));
                    }
                    // Other
                    _ => return Err(anyhow!("Unexpected byte")),
                }
            }
        }
        // List
        108 => {
            let mut list: Vec<Value> = vec![];

            *index += 1;

            loop {
                match data.get(*index).unwrap() {
                    // End character 'e'
                    101 => return Ok(Value::List(list)),
                    // Other
                    _ => list.push(retrieve_and_decode_value(data, index)?),
                }
            }
        }
        // Dictionary
        100 => {
            let mut dictionary: BTreeMap<Vec<u8>, Value> = BTreeMap::new();

            *index += 1;

            let mut key: Option<Vec<u8>> = None;
            loop {
                match data.get(*index).unwrap() {
                    // End character 'e'
                    101 => {
                        if key.is_some() {
                            return Err(anyhow!(
                                "Unexpected end while waiting for dictionary value"
                            ));
                        }

                        return Ok(Value::Dictionary(dictionary));
                    }
                    // Other
                    _ => {
                        let next_value = retrieve_and_decode_value(data, index)?;

                        if key.is_none() {
                            match next_value {
                                Value::ByteString(k) => key = Some(k),
                                _ => {
                                    return Err(anyhow!(
                                        "Key in dictionary should be a byte string"
                                    ))
                                }
                            }
                        } else {
                            dictionary.insert(key.unwrap(), next_value);
                            key = None;
                        }
                    }
                }
            }
        }
        _ => Err(anyhow!("Unexpected byte")),
    }
}
