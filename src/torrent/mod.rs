mod try_from;

#[derive(Debug)]
pub struct Torrent {
    pub info: TorrentInfo,
    pub announce: Vec<u8>,
    pub announce_list: Option<Vec<Vec<Vec<u8>>>>,
    pub creation_date: Option<i64>,
    pub comment: Option<Vec<u8>>,
    pub created_by: Option<Vec<u8>>,
    pub encoding: Option<Vec<u8>>,
}

#[derive(Debug)]
pub enum TorrentInfo {
    SingleFile(TorrentInfoSingleFile),
    MultiFile(TorrentInfoMultiFile),
}

#[derive(Debug)]
pub struct TorrentInfoSingleFile {
    pub piece_length: i64,
    pub pieces: Vec<u8>,
    pub private: Option<bool>,
    pub name: Vec<u8>,
    pub length: i64,
    pub md5sum: Option<Vec<u8>>,
}

#[derive(Debug)]
pub struct TorrentInfoMultiFile {
    pub piece_length: i64,
    pub pieces: Vec<u8>,
    pub private: Option<bool>,
    pub name: Vec<u8>,
    pub files: Vec<File>,
}

#[derive(Debug)]
pub struct File {
    pub length: i64,
    pub md5sum: Option<Vec<u8>>,
    pub path: Vec<Vec<u8>>,
}
