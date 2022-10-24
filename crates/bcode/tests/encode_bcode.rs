mod common;

#[test]
fn encode_bcode() {
    for (left, right) in common::get_comparison_data() {
        assert_eq!(left, bcode::encode(right).unwrap());
    }
}
