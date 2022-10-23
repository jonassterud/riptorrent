use super::*;
use anyhow::{anyhow, Result};
use bcode::map_get;
use std::collections::BTreeMap;

impl Torrent {
    /// Converts a vector of bytes to a `Torrent`.
    ///
    /// # Arguments
    ///
    /// * `vec` - byte vector.
    pub async fn from_bytes(vec: Vec<u8>) -> Result<Torrent> {
        let main_map: BTreeMap<Vec<u8>, bcode::Value> = bcode::decode(&vec, &mut 0)?.try_into()?; // ?
        let info_map: BTreeMap<Vec<u8>, bcode::Value> = map_get(&main_map, "info")?.try_into()?;

        let piece_length: i64 = map_get(&info_map, "piece length")?.try_into()?;
        let pieces: Vec<u8> = map_get(&info_map, "pieces")?.try_into()?;
        let private: Option<bool> = map_get(&info_map, "private")
            .ok()
            .map(|x| -> Result<bool> {
                let temp: i64 = x.try_into()?;
                Ok(temp != 0)
            })
            .transpose()?;

        let info: TorrentInfo = if info_map.contains_key(&b"files".to_vec()) {
            let name: String = map_get(&info_map, "name")?.try_into()?;

            let files_list: Vec<bcode::Value> = map_get(&info_map, "files")?.try_into()?;
            let files = files_list
                .into_iter()
                .map(|file_map| -> Result<File> {
                    let file_map: BTreeMap<Vec<u8>, bcode::Value> = file_map.try_into()?;

                    let length: i64 = map_get(&file_map, "length")?.try_into()?;
                    let md5sum: Option<String> = map_get(&file_map, "md5sum")
                        .ok()
                        .map(|x| -> Result<String> { x.try_into() })
                        .transpose()?;
                    let path: Vec<bcode::Value> = map_get(&file_map, "path")?.try_into()?;
                    let path: Vec<String> = path
                        .into_iter()
                        .map(|x| -> Result<String> { x.try_into() })
                        .collect::<Result<Vec<String>>>()?;

                    Ok(File {
                        length,
                        md5sum,
                        path,
                    })
                })
                .collect::<Result<Vec<File>>>()?;

            TorrentInfo::MultiFileInfo(MultiFileInfo {
                piece_length,
                pieces,
                private,
                name,
                files,
            })
        } else {
            let name: String = map_get(&info_map, "name")?.try_into()?;
            let length: i64 = map_get(&info_map, "length")?.try_into()?;
            let md5sum: Option<String> = map_get(&info_map, "md5sum")
                .ok()
                .map(|x| -> Result<String> { x.try_into() })
                .transpose()?;

            TorrentInfo::SingleFileInfo(SingleFileInfo {
                piece_length,
                pieces,
                private,
                name,
                length,
                md5sum,
            })
        };

        let announce: String = map_get(&main_map, "announce")?.try_into()?;
        let announce_list: Option<Vec<Vec<String>>> = map_get(&main_map, "announce-list")
            .ok()
            .map(|val| -> Result<Vec<Vec<String>>> {
                let outer_list: Vec<bcode::Value> = val.try_into()?;
                outer_list
                    .into_iter()
                    .map(|x| -> Result<Vec<String>> {
                        let inner_list: Vec<bcode::Value> = x.try_into()?;
                        inner_list
                            .into_iter()
                            .map(|x| -> Result<String> { x.try_into() })
                            .collect::<Result<Vec<String>>>()
                    })
                    .collect::<Result<Vec<Vec<String>>>>()
            })
            .transpose()?;
        let creation_date: Option<i64> = map_get(&main_map, "creation date")
            .ok()
            .map(|x| -> Result<i64> { x.try_into() })
            .transpose()?;
        let comment: Option<String> = map_get(&main_map, "comment")
            .ok()
            .map(|x| -> Result<String> { x.try_into() })
            .transpose()?;
        let created_by: Option<String> = map_get(&main_map, "created by")
            .ok()
            .map(|x| -> Result<String> { x.try_into() })
            .transpose()?;
        let encoding: Option<String> = map_get(&main_map, "encoding")
            .ok()
            .map(|x| -> Result<String> { x.try_into() })
            .transpose()?;

        Ok(Torrent {
            info,
            announce,
            announce_list,
            creation_date,
            comment,
            created_by,
            encoding,

            info_hash: sha1_smol::Sha1::from(bcode::encode(map_get(&main_map, "info")?)?)
                .digest()
                .bytes()
                .to_vec(),
        })
    }
}
