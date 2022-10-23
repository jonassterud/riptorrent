mod from_utf8;

#[derive(Debug)]
pub struct Torrent {
    pub info: TorrentInfo,
    pub announce: String,
    pub announce_list: Option<Vec<Vec<String>>>,
    pub creation_date: Option<i64>,
    pub comment: Option<String>,
    pub created_by: Option<String>,
    pub encoding: Option<String>,
}

#[derive(Debug)]
pub enum TorrentInfo {
    SingleFileInfo(SingleFileInfo),
    MultiFileInfo(MultiFileInfo),
}

#[derive(Debug)]
pub struct SingleFileInfo {
    pub piece_length: i64,
    pub pieces: Vec<u8>,
    pub private: Option<bool>,
    pub name: String,
    pub length: i64,
    pub md5sum: Option<String>,
}

#[derive(Debug)]
pub struct MultiFileInfo {
    pub piece_length: i64,
    pub pieces: Vec<u8>,
    pub private: Option<bool>,
    pub name: String,
    pub files: Vec<File>,
}

#[derive(Debug)]
pub struct File {
    pub length: i64,
    pub md5sum: Option<String>,
    pub path: Vec<String>,
}
