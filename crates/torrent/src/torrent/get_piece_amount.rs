use super::*;

impl Torrent {
    /// Get amount of pieces in torrent.
    pub fn get_piece_amount(&self) -> usize {
        match &self.info {
            TorrentInfo::SingleFileInfo(info) => info.pieces.len() % 20,
            TorrentInfo::MultiFileInfo(info) => info.pieces.len() % 20,
        }
    }
}
