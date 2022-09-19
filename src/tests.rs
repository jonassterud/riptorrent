#![cfg(test)]

use crate::bcode::{self, bdecode};
use std::collections::BTreeMap;

#[test]
pub fn bdecode_integer() {
    let left = bdecode::parse::integer(&mut "i32e".as_bytes().to_vec(), &mut 0).unwrap();
    let right = bcode::Value::Integer(32);

    assert_eq!(left, right);
}

#[test]
pub fn bdecode_byte_string() {
    let left = bdecode::parse::byte_string(&mut "4:test".as_bytes().to_vec(), &mut 0).unwrap();
    let right = bcode::Value::ByteString("test".as_bytes().to_vec());

    assert_eq!(left, right);
}

#[test]
pub fn bdecode_list() {
    let left = bdecode::parse::list(&mut "li32e4:teste".as_bytes().to_vec(), &mut 0).unwrap();
    let right = bcode::Value::List(vec![
        bcode::Value::Integer(32),
        bcode::Value::ByteString("test".as_bytes().to_vec()),
    ]);

    assert_eq!(left, right);
}

#[test]
pub fn bdecode_dictionary() {
    let mut dictionary = BTreeMap::new();
    dictionary.insert(
        "key".as_bytes().to_vec(),
        bcode::Value::ByteString("value".as_bytes().to_vec()),
    );

    let left = bdecode::parse::dictionary(&mut "d3:key5:valuee".as_bytes().to_vec(), &mut 0).unwrap();
    let right = bcode::Value::Dictionary(dictionary);

    assert_eq!(left, right);
}
