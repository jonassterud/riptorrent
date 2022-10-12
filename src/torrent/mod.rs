use crate::bcode;

mod try_from;

/// Torrent struct
#[derive(Debug)]
pub struct Torrent {
    pub info: TorrentInfo,
    pub announce: Vec<u8>,
    pub announce_list: Option<Vec<Vec<Vec<u8>>>>,
    pub creation_date: Option<i64>,
    pub comment: Option<Vec<u8>>,
    pub created_by: Option<Vec<u8>>,
    pub encoding: Option<Vec<u8>>,

    pub info_hash: Vec<u8>,
}

/// Torrent info enum
#[derive(Debug)]
pub enum TorrentInfo {
    SingleFile(TorrentInfoSingleFile),
    MultiFile(TorrentInfoMultiFile),
}

/// Torrent info struct for single-file mode
#[derive(Debug)]
pub struct TorrentInfoSingleFile {
    pub piece_length: i64,
    pub pieces: Vec<u8>,
    pub private: Option<bool>,
    pub name: Vec<u8>,
    pub length: i64,
    pub md5sum: Option<Vec<u8>>,
}

/// Torrent info struct for multi-file mode
#[derive(Debug)]
pub struct TorrentInfoMultiFile {
    pub piece_length: i64,
    pub pieces: Vec<u8>,
    pub private: Option<bool>,
    pub name: Vec<u8>,
    pub files: Vec<File>,
}

/// Represents a file in a multi-file mode torrent info
#[derive(Debug)]
pub struct File {
    pub length: i64,
    pub md5sum: Option<Vec<u8>>,
    pub path: Vec<Vec<u8>>,
}

impl Torrent {
    /// Get total number of bytes that this torrent download contains
    pub fn get_left(&self) -> i64 {
        match &self.info {
            TorrentInfo::SingleFile(info) => info.length,
            TorrentInfo::MultiFile(info) => info.files.iter().map(|x| x.length).sum(),
        }
    }
}
