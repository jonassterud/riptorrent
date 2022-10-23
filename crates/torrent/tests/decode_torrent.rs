mod common;

#[test]
fn decode_torrent() {
    let data = common::read_torrent().unwrap();
    torrent::Torrent::from_utf8(data).unwrap();
}
