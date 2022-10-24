mod from_bytes;
mod get_size;

/// Struct representing a torrent file.
#[derive(Debug)]
pub struct Torrent {
    pub info: TorrentInfo,
    pub announce: String,
    pub announce_list: Option<Vec<Vec<String>>>,
    pub creation_date: Option<i64>,
    pub comment: Option<String>,
    pub created_by: Option<String>,
    pub encoding: Option<String>,

    pub info_hash: Vec<u8>,
}

/// Info part of a torrent.
#[derive(Debug)]
pub enum TorrentInfo {
    SingleFileInfo(SingleFileInfo),
    MultiFileInfo(MultiFileInfo),
}

/// Info for a single-file torrent.
#[derive(Debug)]
pub struct SingleFileInfo {
    pub piece_length: i64,
    pub pieces: Vec<u8>,
    pub private: Option<bool>,
    pub name: String,
    pub length: i64,
    pub md5sum: Option<String>,
}

/// Info for a multi-file torrent.
#[derive(Debug)]
pub struct MultiFileInfo {
    pub piece_length: i64,
    pub pieces: Vec<u8>,
    pub private: Option<bool>,
    pub name: String,
    pub files: Vec<File>,
}

/// Struct representing a file in a multi-file torrent.
#[derive(Debug)]
pub struct File {
    pub length: i64,
    pub md5sum: Option<String>,
    pub path: Vec<String>,
}
