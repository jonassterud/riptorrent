mod common;

use builder::Builder;

#[test]
fn create_builder_and_find_blocks() {
    let piece_amount = 1168;
    let mut bitfield = vec![0_u8; piece_amount / 8];
    bitfield[4] ^= 1_u8 << 7; // flip the (4 * 8)th bit.
    let index = 4 * 8;

    let mut builder = Builder::new(piece_amount, 2097152, u32::pow(2, 14) as usize);
    let block = builder.take_missing_relevant_block(&bitfield).unwrap();

    assert_eq!(index, block.index);
}
