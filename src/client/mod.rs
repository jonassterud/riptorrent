use crate::Torrent;
use crate::bcode;

use anyhow::{anyhow, Result};
use sha1::{Digest, Sha1};

/// Client struct
#[derive(Debug)]
pub struct Client {
    pub request_parameters: TrackerRequestParameters,
    pub last_response: Option<TrackerResponse>,
    pub torrent: Torrent,
}

impl Client {
    /// Construct a new `Client`
    pub fn new(torrent: Torrent) -> Result<Client> {
        Ok(Client {
            request_parameters: TrackerRequestParameters::from_torrent(&torrent)?,
            last_response: None,
            torrent,
        })
    }
}

/// Tracker request parameters
#[derive(Debug)]
pub struct TrackerRequestParameters {
    pub info_hash: Vec<u8>,
    pub peer_id: String,
    pub port: String,
    pub uploaded: String,
    pub downloaded: String,
    pub left: String,
    pub compact: String,
    pub no_peer_id: String,
    pub event: String,
    pub ip: String,
    pub numwant: String,
    pub key: String,
    pub trackerid: String,
}

impl TrackerRequestParameters {
    /// Construct a new `TrackerRequestParameters` from a torrent
    ///
    /// # Arguments
    ///
    /// * `torrent` - a reference to a `Torrent` struct
    pub fn from_torrent(torrent: &Torrent) -> Result<TrackerRequestParameters> {
        let info_dict = bcode::encode(vec![&torrent.info_dict])?;
        let mut hasher = Sha1::new();
        hasher.update(info_dict);

        Ok(TrackerRequestParameters {
            info_hash: hasher.finalize().to_vec(),
            peer_id: todo!(),
            port: todo!(),
            uploaded: todo!(),
            downloaded: todo!(),
            left: todo!(),
            compact: todo!(),
            no_peer_id: todo!(),
            event: todo!(),
            ip: todo!(),
            numwant: todo!(),
            key: todo!(),
            trackerid: todo!(),
        })
    }
}

/// Tracker response
#[derive(Debug)]
pub struct TrackerResponse {
    pub failure_reason: Option<String>,
    pub warning_message: Option<String>,
    pub interval: Option<u32>,
    pub min_interval: Option<u32>,
    pub tracker_id: Option<String>,
    pub complete: Option<u64>,
    pub incomplete: Option<u64>,
    pub peers: Option<Peers>,
}

/// Enum to represent peers
#[derive(Debug)]
pub enum Peers {
    Dictionary(PeersDictionaryModel),
    Binary(String),
}

/// Struct to represent a peer in dictionary model
#[derive(Debug)]
pub struct PeersDictionaryModel {
    pub peer_id: Option<String>,
    pub ip: Option<String>,
    pub port: Option<u16>,
}
