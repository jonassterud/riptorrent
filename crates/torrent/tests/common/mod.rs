use anyhow::Result;

pub fn read_torrent() -> Result<Vec<u8>> {
    let path = std::env::current_dir()?;
    let path = path.join("/workspaces/riptorrent/crates/torrent/tests/common/ubuntu-22.04.1-desktop-amd64.iso.torrent");

    Ok(std::fs::read(path)?)
}