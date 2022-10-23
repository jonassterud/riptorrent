use super::*;

impl Torrent {
    pub fn get_left(&self) -> i64 {
        match &self.info {
            TorrentInfo::SingleFileInfo(info) => info.length,
            TorrentInfo::MultiFileInfo(info) => info.files.iter().map(|x| x.length).sum(),
        }
    }
}
