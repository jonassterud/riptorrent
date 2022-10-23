mod common;

#[test]
fn decode_torrent() {
    let data =
        std::fs::read("../../media/torrents/ubuntu-22.04.1-desktop-amd64.iso.torrent").unwrap();
    // println!("{:?}", data);
    println!("{:?}", torrent::Torrent::from_utf8(data).unwrap());
}
