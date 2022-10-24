mod common;

#[test]
fn decode_bcode() {
    for (left, right) in common::get_comparison_data() {
        assert_eq!(bcode::decode(left, &mut 0_usize).unwrap(), right);
    }
}
