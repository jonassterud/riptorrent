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

impl From<i64> for Value {
    fn from(integer: i64) -> Self {
        Value::Integer(integer)
    }
}

impl TryFrom<Value> for i64 {
    type Error = anyhow::Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Integer(out) => Ok(out),
            _ => Err(anyhow!("Value is not a integer")),
        }
    }
}

impl From<Vec<u8>> for Value {
    fn from(byte_string: Vec<u8>) -> Self {
        Value::ByteString(byte_string)
    }
}

impl TryFrom<Value> for Vec<u8> {
    type Error = anyhow::Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::ByteString(out) => Ok(out),
            _ => Err(anyhow!("Value is not a byte string")),
        }
    }
}

impl From<String> for Value {
    fn from(string: String) -> Self {
        Value::ByteString(string.into_bytes())
    }
}

impl TryFrom<Value> for String {
    type Error = anyhow::Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::ByteString(out) => Ok(String::from_utf8(out)?),
            _ => Err(anyhow!("Value is not a byte string")),
        }
    }
}

impl From<Vec<Value>> for Value {
    fn from(list: Vec<Value>) -> Self {
        Value::List(list)
    }
}

impl TryFrom<Value> for Vec<Value> {
    type Error = anyhow::Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::List(out) => Ok(out),
            _ => Err(anyhow!("Value is not a list")),
        }
    }
}

impl From<BTreeMap<Vec<u8>, Value>> for Value {
    fn from(dictionary: BTreeMap<Vec<u8>, Value>) -> Self {
        Value::Dictionary(dictionary)
    }
}

impl TryFrom<Value> for BTreeMap<Vec<u8>, Value> {
    type Error = anyhow::Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Dictionary(out) => Ok(out),
            _ => Err(anyhow!("Value is not a dictionary")),
        }
    }
}

/// Get value from dictionary, returns `None` if not found.
///
/// # Arguments
///
/// * `map` - `BTreeMap` to search trough.
/// * `key` - key to search for.
pub fn map_get(map: &BTreeMap<Vec<u8>, Value>, key: &str) -> Result<Value> {
    map.get(key.as_bytes())
        .ok_or_else(|| anyhow!("Could not find \"{key}\" in map"))
        .cloned()
}