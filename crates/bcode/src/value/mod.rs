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

impl From<Value> for i64 {
    fn from(val: Value) -> Self {
        val.into()
    }
}

impl From<Vec<u8>> for Value {
    fn from(byte_string: Vec<u8>) -> Self {
        Value::ByteString(byte_string)
    }
}

impl From<Value> for Vec<u8> {
    fn from(val: Value) -> Self {
        val.into()
    }
}

impl From<String> for Value {
    fn from(string: String) -> Self {
        Value::ByteString(string.into_bytes())
    }
}

impl From<Value> for String {
    fn from(val: Value) -> Self {
        val.into()
    }
}

impl From<Vec<Value>> for Value {
    fn from(list: Vec<Value>) -> Self {
        Value::List(list)
    }
}

impl From<Value> for Vec<Value> {
    fn from(val: Value) -> Self {
        val.into()
    }
}

impl From<BTreeMap<Vec<u8>, Value>> for Value {
    fn from(dictionary: BTreeMap<Vec<u8>, Value>) -> Self {
        Value::Dictionary(dictionary)
    }
}

impl From<Value> for BTreeMap<Vec<u8>, Value> {
    fn from(val: Value) -> Self {
        val.into()
    }
}
