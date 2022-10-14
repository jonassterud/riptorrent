use super::*;
use anyhow::{anyhow, Result};
use bcode::Value;
use peer::Peer;

/// Tracker response
#[derive(Debug)]
pub struct TrackerResponse {
    pub interval: Option<i64>,
    pub min_interval: Option<i64>,
    pub tracker_id: Option<Vec<u8>>,
    pub complete: Option<i64>,
    pub incomplete: Option<i64>,
    pub peers: Option<Vec<Peer>>,
}

impl TrackerResponse {
    /// Create a new TrackerResponse from bytes
    pub fn from_bytes(mut data: Vec<u8>) -> Result<TrackerResponse> {
        let dict = bcode::decode(&mut data, &mut 0)?
            .get(0)
            .ok_or_else(|| anyhow!("Unexpected data"))?
            .get_inner_dictionary()?;

        if dict.contains_key(&b"failure reason".to_vec()) {
            return Err(anyhow!(
                "Error: {:?}",
                dict.get(&b"failure reason".to_vec()).unwrap()
            ));
        }

        if dict.contains_key(&b"warning message".to_vec()) {
            println!(
                "Warning: {:?}",
                dict.get(&b"warning message".to_vec()).unwrap()
            );
        }

        let interval = dict
            .get(&b"interval".to_vec())
            .map(|e| e.get_inner_integer().unwrap());
        let min_interval = dict
            .get(&b"min interval".to_vec())
            .map(|e| e.get_inner_integer().unwrap());
        let tracker_id = dict
            .get(&b"tracker id".to_vec())
            .map(|e| e.get_inner_byte_string().unwrap());
        let complete = dict
            .get(&b"complete".to_vec())
            .map(|e| e.get_inner_integer().unwrap());
        let incomplete = dict
            .get(&b"incomplete".to_vec())
            .map(|e| e.get_inner_integer().unwrap());
        let mut peers = vec![];

        match dict.get(&b"peers".to_vec()) {
            Some(Value::List(peers_dm)) => {
                for peer_d in peers_dm {
                    let peer_d = peer_d.get_inner_dictionary()?;

                    let peer_id = peer_d
                        .get(&b"peer id".to_vec())
                        .map(|e| e.get_inner_byte_string().unwrap());
                    let ip = peer_d
                        .get(&b"ip".to_vec())
                        .map(|e| {
                            let temp = e.get_inner_byte_string().unwrap();
                            *array_ref![temp, 0, 4]
                        })
                        .unwrap();
                    let port = peer_d
                        .get(&b"port".to_vec())
                        .map(|e| e.get_inner_integer().unwrap() as u16)
                        .unwrap();

                    peers.push(Peer {
                        peer_id,
                        ip,
                        port,
                        stream: None,
                    });
                }
            }
            Some(Value::ByteString(peers_bm)) => {
                for chunk in peers_bm.chunks(6) {
                    let ip = Some(*array_ref![chunk, 0, 4]).unwrap();
                    let port = Some(u16::from_be_bytes(*array_ref![chunk, 4, 2])).unwrap();

                    peers.push(Peer {
                        peer_id: None,
                        ip,
                        port,
                        stream: None,
                    });
                }
            }
            _ => return Err(anyhow!("Couldn't decode peers")),
        };

        Ok(TrackerResponse {
            interval,
            min_interval,
            tracker_id,
            complete,
            incomplete,
            peers: Some(peers),
        })
    }
}
