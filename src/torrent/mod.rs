use crate::Value;

use anyhow::{anyhow, Result};
use std::convert::TryFrom;

impl TryFrom<&Value> for Torrent {
    type Error = anyhow::Error;

    // TODO: Rewrite this..
    fn try_from(value: &Value) -> Result<Self> {
        /// Helper functions to minimize code repetition for decode::Value::Dictionary(...)
        impl Value {
            fn try_get_v(&self, key: &str) -> Result<Option<Value>> {
                if let Value::Dictionary(dict) = &self {
                    let byte_string = key.as_bytes().to_vec();
                    let dict_value = dict.get(&byte_string).cloned();

                    Ok(dict_value)
                } else {
                    Err(anyhow!("Value is not a dictionary"))
                }
            }

            fn get_v(&self, key: &str) -> Result<Value> {
                if let Value::Dictionary(dict) = &self {
                    let byte_string = key.as_bytes().to_vec();
                    let dict_value = dict.get(&byte_string).cloned();

                    Ok(dict_value.ok_or_else(|| anyhow!(format!("Missing \"{}\" key", key)))?)
                } else {
                    Err(anyhow!("Value is not a dictionary"))
                }
            }
        }

        let announce = value.get_v("announce")?;
        let announce_list = value.try_get_v("announce-list")?;
        let creation_date = value.try_get_v("creation date")?;
        let comment = value.try_get_v("comment")?;
        let created_by = value.try_get_v("created by")?;
        let encoding = value.try_get_v("encoding")?;

        let info_dict = value.get_v("info")?;
        let piece_length = info_dict.get_v("piece length")?;
        let pieces = info_dict.get_v("pieces")?;
        let private = info_dict.try_get_v("private")?;

        // Declare torrent info either in single- or multi file mode
        let torrent_info = if info_dict.try_get_v("files")?.is_none() {
            let name = info_dict.get_v("name")?;
            let length = info_dict.get_v("length")?;
            let md5sum = info_dict.try_get_v("md5sum")?;

            TorrentInfo::SingleFile(TorrentInfoSingleFile {
                piece_length: piece_length.get_inner_integer()?,
                pieces: pieces.get_inner_byte_string()?,
                private: private.map(|e| e.get_inner_integer().ok() > Some(0)),
                name: name.get_inner_byte_string()?,
                length: length.get_inner_integer()?,
                md5sum: md5sum.map(|e| e.get_inner_byte_string()).transpose()?,
            })
        } else {
            let name = info_dict.get_v("name")?;
            let mut files: Vec<File> = vec![];
            for file_dict in info_dict.get_v("files")?.get_inner_list()? {
                let length = file_dict.get_v("length")?;
                let md5sum = file_dict.try_get_v("md5sum")?;
                let path = file_dict.get_v("path")?;

                files.push(File {
                    length: length.get_inner_integer()?,
                    md5sum: md5sum.map(|e| e.get_inner_byte_string()).transpose()?,
                    path: path
                        .get_inner_list()?
                        .iter()
                        .map(|e| e.get_inner_byte_string())
                        .collect::<Result<Vec<Vec<u8>>>>()?,
                });
            }

            TorrentInfo::MultiFile(TorrentInfoMultiFile {
                piece_length: piece_length.get_inner_integer()?,
                pieces: pieces.get_inner_byte_string()?,
                private: private.map(|e| e.get_inner_integer().ok() > Some(0)),
                name: name.get_inner_byte_string()?,
                files,
            })
        };

        Ok(Torrent {
            info: torrent_info,
            announce: announce.get_inner_byte_string()?,
            announce_list: announce_list
                .map(|e| -> Result<Vec<Vec<Vec<u8>>>> {
                    // ugh
                    e.get_inner_list()
                        .unwrap()
                        .iter()
                        .map(|k| -> Result<Vec<Vec<u8>>> {
                            k.get_inner_list()
                                .unwrap_or_default()
                                .iter()
                                .map(|d| d.get_inner_byte_string())
                                .collect()
                        })
                        .collect()
                })
                .transpose()?,
            creation_date: creation_date.map(|e| e.get_inner_integer()).transpose()?,
            comment: comment.map(|e| e.get_inner_byte_string()).transpose()?,
            created_by: created_by.map(|e| e.get_inner_byte_string()).transpose()?,
            encoding: encoding.map(|e| e.get_inner_byte_string()).transpose()?,
        })
    }
}

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
