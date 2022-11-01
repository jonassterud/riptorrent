mod common;

use builder::{Block, Builder};

#[test]
fn get_finished_block() {
    let mut builder = Builder::new(1168, 2097152, u32::pow(2, 14) as usize);
    builder.finished = builder.missing.clone();

    let wanted_block = Block {
        index: 4,
        begin: 0,
        data: vec![0; 25],
    };

    let finished_block = builder.get_finished_block(4, 0, 25).unwrap();

    assert_eq!(wanted_block, finished_block);
}