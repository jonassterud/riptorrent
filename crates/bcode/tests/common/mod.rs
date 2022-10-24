/// Returns a list of comparison data for bcode.
/// The left element in each tuple is the raw bcode, and the
/// right element is the decoded bcode as a BV enum.
pub fn get_comparison_data() -> Vec<(&'static [u8], bcode::Value)> {
    vec![
        (b"i42e", bcode::Value::Integer(42)),
        (b"4:test", bcode::Value::ByteString(b"test".to_vec())),
        (
            b"li42e4:teste",
            bcode::Value::List(vec![
                bcode::Value::Integer(42),
                bcode::Value::ByteString(b"test".to_vec()),
            ]),
        ),
        (
            b"d6:test_1i42e6:test_2i42ee",
            bcode::Value::Dictionary(maplit::btreemap! {
                b"test_1".to_vec() => bcode::Value::Integer(42),
                b"test_2".to_vec() => bcode::Value::Integer(42),
            }),
        ),
        // ++
    ]
}
