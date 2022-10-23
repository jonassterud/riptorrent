mod common;

#[test]
fn decode_torrent() {
    let path = std::env::current_dir().unwrap();
    let path = path.join("/workspaces/riptorrent/crates/torrent/tests/common/ubuntu-22.04.1-desktop-amd64.iso.torrent");
    let data = std::fs::read(path).unwrap();

    torrent::Torrent::from_utf8(data).unwrap();
}
