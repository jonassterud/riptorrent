pub mod message;

mod protocol;
mod request;
mod response;

use crate::bcode;
use crate::Torrent;

use request::*;
use response::*;

use anyhow::{anyhow, Result};

/// Client struct
#[derive(Debug)]
pub struct Client {
    pub am_choking: bool,
    pub am_interested: bool,
    pub peer_choking: bool,
    pub peer_interested: bool,
    pub request_parameters: TrackerRequestParameters,
    pub last_response: Option<TrackerResponse>,
    pub torrent: Torrent,
}

impl Client {
    /// Construct a new `Client`
    pub fn new(torrent: Torrent) -> Result<Client> {
        Ok(Client {
            am_choking: true,
            am_interested: false,
            peer_choking: true,
            peer_interested: false,
            request_parameters: TrackerRequestParameters::from_torrent(&torrent)?,
            last_response: None,
            torrent,
        })
    }

    /// Get handshake message
    pub fn get_handshake(mut info_hash: Vec<u8>, mut peer_id: Vec<u8>) -> Result<Vec<u8>> {
        let mut out = vec![];

        out.push(19);
        out.append(&mut "BitTorrent protocol".as_bytes().to_vec());
        out.append(&mut vec![0; 8]);
        out.append(&mut info_hash);
        out.append(&mut peer_id);

        Ok(out)
    }
}
