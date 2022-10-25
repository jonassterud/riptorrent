use super::*;

impl Torrent {
    /// Get length of piece.
    pub fn get_piece_length(&self) -> i64 {
        match &self.info {
            TorrentInfo::SingleFileInfo(info) => info.piece_length,
            TorrentInfo::MultiFileInfo(info) => info.piece_length,
        }
    }
}
