use super::{File, Torrent, TorrentInfo, TorrentInfoMultiFile, TorrentInfoSingleFile};
use crate::bcode;

use anyhow::{anyhow, Result};
use sha1_smol::Sha1;
use std::convert::TryFrom;

impl bcode::Value {
    /// Get value from dictionary, returns `None` if not found
    ///
    /// # Arguments
    ///
    /// * `key` - dictionary key
    fn try_get_v(&self, key: &str) -> Result<Option<bcode::Value>> {
        let byte_string = key.as_bytes().to_vec();
        let dict = self.get_inner_dictionary()?;
        let dict_value = dict.get(&byte_string).cloned();

        Ok(dict_value)
    }

    /// Get value from dictionary, returns `Error` if not found
    ///
    /// # Arguments
    ///
    /// * `key` - dictionary key
    fn get_v(&self, key: &str) -> Result<bcode::Value> {
        let byte_string = key.as_bytes().to_vec();
        let dict = self.get_inner_dictionary()?;
        let dict_value = dict.get(&byte_string).cloned();

        dict_value.ok_or_else(|| anyhow!(format!("Missing \"{}\" key", key)))
    }
}

impl TryFrom<&mut Vec<u8>> for Torrent {
    type Error = anyhow::Error;

    // TODO: Rewrite this..
    fn try_from(data: &mut Vec<u8>) -> Result<Self> {
        let value = bcode::decode(data, &mut 0)?;
        let value = value.get(0).ok_or_else(|| anyhow!("Failed decoding"))?;

        let announce = value.get_v("announce")?;
        let announce_list = value.try_get_v("announce-list")?;
        let creation_date = value.try_get_v("creation date")?;
        let comment = value.try_get_v("comment")?;
        let created_by = value.try_get_v("created by")?;
        let encoding = value.try_get_v("encoding")?;

        let info_dict = value.get_v("info")?;
        let torrent_info = TorrentInfo::try_from(&info_dict)?;

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

            info_hash: Sha1::from(bcode::encode(vec![&info_dict])?)
                .digest()
                .bytes()
                .to_vec(),
        })
    }
}

impl TryFrom<&bcode::Value> for TorrentInfo {
    type Error = anyhow::Error;

    fn try_from(value: &bcode::Value) -> Result<Self> {
        let piece_length = value.get_v("piece length")?;
        let pieces = value.get_v("pieces")?;
        let private = value.try_get_v("private")?;

        if value.try_get_v("files")?.is_none() {
            let name = value.get_v("name")?;
            let length = value.get_v("length")?;
            let md5sum = value.try_get_v("md5sum")?;

            Ok(TorrentInfo::SingleFile(TorrentInfoSingleFile {
                piece_length: piece_length.get_inner_integer()?,
                pieces: pieces.get_inner_byte_string()?,
                private: private.map(|e| e.get_inner_integer().ok() > Some(0)),
                name: name.get_inner_byte_string()?,
                length: length.get_inner_integer()?,
                md5sum: md5sum.map(|e| e.get_inner_byte_string()).transpose()?,
            }))
        } else {
            let name = value.get_v("name")?;
            let mut files: Vec<File> = vec![];
            for file_dict in value.get_v("files")?.get_inner_list()? {
                files.push(File::try_from(&file_dict)?);
            }

            Ok(TorrentInfo::MultiFile(TorrentInfoMultiFile {
                piece_length: piece_length.get_inner_integer()?,
                pieces: pieces.get_inner_byte_string()?,
                private: private.map(|e| e.get_inner_integer().ok() > Some(0)),
                name: name.get_inner_byte_string()?,
                files,
            }))
        }
    }
}

impl TryFrom<&bcode::Value> for File {
    type Error = anyhow::Error;

    fn try_from(value: &bcode::Value) -> Result<Self> {
        let length = value.get_v("length")?;
        let md5sum = value.try_get_v("md5sum")?;
        let path = value.get_v("path")?;

        Ok(File {
            length: length.get_inner_integer()?,
            md5sum: md5sum.map(|e| e.get_inner_byte_string()).transpose()?,
            path: path
                .get_inner_list()?
                .iter()
                .map(|e| e.get_inner_byte_string())
                .collect::<Result<Vec<Vec<u8>>>>()?,
        })
    }
}
