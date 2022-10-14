use super::*;

use anyhow::{anyhow, Result};
use std::io::prelude::*;
use std::net::TcpStream;

impl Client {
    pub fn send_tracker_request(&self) -> Result<()> {
        let params = TrackerRequestParameters::from_torrent(&self.torrent)?;
        let announce = String::from_utf8(self.torrent.announce.clone())?;
        let final_url = format!("{}?{}", announce, params.as_url_params());

        println!("{}", final_url);
        let response = reqwest::blocking::get(final_url)?.bytes()?.to_vec();
        let tracker_response = TrackerResponse::from_bytes(response)?;

        println!("{:?}", tracker_response);

        Ok(())
    }
}
