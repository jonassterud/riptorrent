#![cfg(test)]

use crate::decode::{parse, Value};
use std::collections::BTreeMap;

#[test]
pub fn parse_integer() {
    let left = parse::integer(&mut "i32e".as_bytes().to_vec(), &mut 0).unwrap();
    let right = Value::Integer(32);

    assert_eq!(left, right);
}

#[test]
pub fn parse_byte_string() {
    let left = parse::byte_string(&mut "4:test".as_bytes().to_vec(), &mut 0).unwrap();
    let right = Value::ByteString("test".as_bytes().to_vec());

    assert_eq!(left, right);
}

#[test]
pub fn parse_list() {
    let left = parse::list(&mut "li32e4:teste".as_bytes().to_vec(), &mut 0).unwrap();
    let right = Value::List(vec![
        Value::Integer(32),
        Value::ByteString("test".as_bytes().to_vec()),
    ]);

    assert_eq!(left, right);
}

#[test]
pub fn parse_dictionary() {
    let mut dictionary = BTreeMap::new();
    dictionary.insert(
        "key".as_bytes().to_vec(),
        Value::ByteString("value".as_bytes().to_vec()),
    );

    let left = parse::dictionary(&mut "d3:key5:valuee".as_bytes().to_vec(), &mut 0).unwrap();
    let right = Value::Dictionary(dictionary);

    assert_eq!(left, right);
}
