use anyhow::{anyhow, Result};
use std::collections::BTreeMap;

/// Data values supported by bcode.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    Integer(i64),
    ByteString(Vec<u8>),
    List(Vec<Value>),
    Dictionary(BTreeMap<Vec<u8>, Value>),
}

impl Value {
    /// Get the inner value of `Value::Integer(i64)`.
    pub fn get_inner_integer(&self) -> Result<i64> {
        if let Value::Integer(inner) = self {
            Ok(inner.to_owned())
        } else {
            Err(anyhow!("{:?} is not a `Value::Integer`", self))
        }
    }

    /// Get the inner value of `Value::ByteString(Vec<u8>)`.
    pub fn get_inner_byte_string(&self) -> Result<Vec<u8>> {
        if let Value::ByteString(inner) = self {
            Ok(inner.to_owned())
        } else {
            Err(anyhow!("{:?} is not a `Value::ByteString`", self))
        }
    }

    /// Get the inner value of `Value::List(Vec<Value>)`.
    pub fn get_inner_list(&self) -> Result<Vec<Value>> {
        if let Value::List(inner) = self {
            Ok(inner.to_owned())
        } else {
            Err(anyhow!("{:?} is not a `Value::List`", self))
        }
    }

    /// Get the inner value of `Value::Dictionary(BTreeMap<Vec<u8>, Value>)`.
    pub fn get_inner_dictionary(&self) -> Result<BTreeMap<Vec<u8>, Value>> {
        if let Value::Dictionary(inner) = self {
            Ok(inner.to_owned())
        } else {
            Err(anyhow!("{:?} is not a `Value::Dictionary`", self))
        }
    }
}
