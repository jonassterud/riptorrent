pub mod message;

mod peer;
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
    pub last_response: Option<TrackerResponse>,
    pub torrent: Torrent,
}

impl Client {
    /// Construct a new `Client`
    pub fn new(torrent: Torrent) -> Result<Client> {
        Ok(Client {
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

    /// Send request to tracker
    pub fn send_tracker_request(&mut self) -> Result<()> {
        let params = TrackerRequestParameters::from_torrent(&self.torrent)?;
        let announce = String::from_utf8(self.torrent.announce.clone())?;
        let final_url = format!("{}?{}", announce, params.as_url_params());

        let response = reqwest::blocking::get(final_url)?.bytes()?.to_vec();
        let mut tracker_response = TrackerResponse::from_bytes(response)?;
        
        if let Some(peers) = tracker_response.peers.as_mut() {
            for peer in peers {
                peer.info_hash = Some(params.info_hash.clone());
            }
        }

        self.last_response = Some(tracker_response);

        Ok(())
    }
}
