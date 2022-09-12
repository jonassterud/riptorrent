use crate::decode::{decode, Value};

#[allow(dead_code)]
pub fn get_comparison_data() -> Vec<(&'static str, Value)> {
    vec![
        ("i5944e", Value::Integer(5944)),
        ("i0e", Value::Integer(0)),
        ("i-50e", Value::Integer(-50)),
        ("3:abc", Value::ByteString(vec![97, 98, 99])),
        ("0:", Value::ByteString(vec![])),
        ("li573e3:abce", Value::List(vec![Value::Integer(573), Value::ByteString(vec![97, 98, 99])])),
        //("d3:abci573e3:cbai375ee", Value::Dictionary(btreemap!(vec![97, 98, 99] => BVal::Number(573), vec![99, 98, 97] => BVal::Number(375)))),
    ]
}

#[test]
fn decode_bencode() {
    for (d, e) in get_comparison_data() {
        let left = decode(d.as_bytes().to_vec()).unwrap();
        let right = e;

        if *left.first().unwrap() != right {
            panic!("failed decoding, {:?} != {:?}", left, right);
        }
    }
}
