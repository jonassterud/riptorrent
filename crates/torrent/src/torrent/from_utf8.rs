use super::*;
use anyhow::{anyhow, Result};
use std::collections::BTreeMap;

/// Get value from dictionary, returns `None` if not found
///
/// # Arguments
///
/// * `map` - `BTreeMap` to search trough
/// * `key` - key to search for
fn get(map: &BTreeMap<Vec<u8>, bcode::Value>, key: &str) -> Result<bcode::Value> {
    map.get(key.as_bytes())
        .ok_or_else(|| anyhow!("Could not find \"{key}\" in map"))
        .cloned()
}

impl Torrent {
    pub fn from_utf8(vec: Vec<u8>) -> Result<Torrent> {
        let main_map: BTreeMap<Vec<u8>, bcode::Value> = bcode::decode(&vec, &mut 0)?.into(); // ?
        let info_map: BTreeMap<Vec<u8>, bcode::Value> = get(&main_map, "info")?.into();

        let piece_length: i64 = get(&info_map, "piece length")?.into();
        let pieces: Vec<u8> = get(&info_map, "piece length")?.into();
        let private: Option<bool> = get(&info_map, "private").ok().map(|x| {
            let temp: i64 = x.into();
            temp != 0
        });

        let info: TorrentInfo = if info_map.contains_key(&b"files".to_vec()) {
            let name: String = get(&info_map, "name")?.into();

            let files_list: Vec<bcode::Value> = get(&main_map, "files")?.into();
            let files = files_list
                .into_iter()
                .map(|file_map| -> Result<File> {
                    let file_map: BTreeMap<Vec<u8>, bcode::Value> = file_map.into();

                    let length: i64 = get(&file_map, "length")?.into();
                    let md5sum: Option<String> = get(&file_map, "md5sum").ok().map(|x| x.into());
                    let path: Vec<bcode::Value> = get(&file_map, "path")?.into();
                    let path: Vec<String> =
                        path.into_iter().map(|x| -> String { x.into() }).collect();

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
            let name: String = get(&info_map, "name")?.into();
            let length: i64 = get(&info_map, "length")?.into();
            let md5sum: Option<String> = get(&info_map, "md5sum").ok().map(|x| x.into());

            TorrentInfo::SingleFileInfo(SingleFileInfo {
                piece_length,
                pieces,
                private,
                name,
                length,
                md5sum,
            })
        };

        let announce: String = get(&main_map, "announce")?.into();
        let announce_list: Option<Vec<Vec<String>>> =
            get(&main_map, "announce-list")
                .ok()
                .map(|val| -> Vec<Vec<String>> {
                    let outer_list: Vec<bcode::Value> = val.into();
                    outer_list
                        .into_iter()
                        .map(|x| -> Vec<String> {
                            let inner_list: Vec<bcode::Value> = x.into();
                            inner_list
                                .into_iter()
                                .map(|x| -> String { x.into() })
                                .collect()
                        })
                        .collect()
                });
        let creation_date: Option<i64> = get(&main_map, "creation date").ok().map(|x| x.into());
        let comment: Option<String> = get(&main_map, "comment").ok().map(|x| x.into());
        let created_by: Option<String> = get(&main_map, "created by").ok().map(|x| x.into());
        let encoding: Option<String> = get(&main_map, "encoding").ok().map(|x| x.into());

        Ok(Torrent {
            info,
            announce,
            announce_list,
            creation_date,
            comment,
            created_by,
            encoding,
        })
    }
}
