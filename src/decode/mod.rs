pub mod parse;

use anyhow::{anyhow, Result};
use std::collections::BTreeMap;

/// Represents the possible bencoded values
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Integer(i64),
    ByteString(Vec<u8>),
    List(Vec<Value>),
    Dictionary(BTreeMap<Vec<u8>, Value>),
}

impl Value {
    /// Check if two enum are of the same variant
    pub fn variant_eq(&self, other: &Value) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(other)
    }

    pub fn get_inner_integer(&self) -> Result<i64> {
        if let Value::Integer(inner) = self {
            Ok(inner.to_owned())
        } else {
            Err(anyhow!("Value is not a \"Integer\""))
        }
    }

    pub fn get_inner_byte_string(&self) -> Result<Vec<u8>> {
        if let Value::ByteString(inner) = self {
            Ok(inner.to_owned())
        } else {
            Err(anyhow!("Value is not a \"ByteString\""))
        }
    }

    pub fn get_inner_list(&self) -> Result<Vec<Value>> {
        if let Value::List(inner) = self {
            Ok(inner.to_owned())
        } else {
            Err(anyhow!("Value is not a \"List\""))
        }
    }

    pub fn get_inner_dictionary(&self) -> Result<BTreeMap<Vec<u8>, Value>> {
        if let Value::Dictionary(inner) = self {
            Ok(inner.to_owned())
        } else {
            Err(anyhow!("Value is not a \"Dictionary\""))
        }
    }
}

/// Decodes bencoded data
///
/// # Arguments
///
/// * `data` - mutable reference to data to decode
/// * `index` - mutable reference to the (data) index
pub fn decode(data: &mut [u8], index: &mut usize) -> Result<Vec<Value>> {
    let mut out = vec![];

    while *index + 1 < data.len() {
        out.push(parse::any(data, index)?);
    }

    Ok(out)
}
