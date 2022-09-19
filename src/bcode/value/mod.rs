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
    /// Checks if two `Value` enums are of the same variant
    ///
    /// # Arguments
    ///
    /// * `other` - `Value` enum to compare to
    pub fn variant_eq(&self, other: &Value) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(other)
    }

    /// Get the inner value of Value::Integer(i64)
    pub fn get_inner_integer(&self) -> Result<i64> {
        if let Value::Integer(inner) = self {
            Ok(inner.to_owned())
        } else {
            Err(anyhow!("Value is not a \"Integer\""))
        }
    }

    /// Get the inner value of Value::ByteString(Vec<u8>)
    pub fn get_inner_byte_string(&self) -> Result<Vec<u8>> {
        if let Value::ByteString(inner) = self {
            Ok(inner.to_owned())
        } else {
            Err(anyhow!("Value is not a \"ByteString\""))
        }
    }

    /// Get the inner value of Value::List(Vec<Value>)
    pub fn get_inner_list(&self) -> Result<Vec<Value>> {
        if let Value::List(inner) = self {
            Ok(inner.to_owned())
        } else {
            Err(anyhow!("Value is not a \"List\""))
        }
    }

    /// Get the inner value of Value::Dictionary(BTreeMap<Vec<u8>, Value>)
    pub fn get_inner_dictionary(&self) -> Result<BTreeMap<Vec<u8>, Value>> {
        if let Value::Dictionary(inner) = self {
            Ok(inner.to_owned())
        } else {
            Err(anyhow!("Value is not a \"Dictionary\""))
        }
    }
}
