use super::*;

impl Torrent {
    /// Get amount of bytes in torrent.
    pub fn get_size(&self) -> i64 {
        match &self.info {
            TorrentInfo::SingleFileInfo(info) => info.length,
            TorrentInfo::MultiFileInfo(info) => info.files.iter().map(|x| x.length).sum(),
        }
    }
}
