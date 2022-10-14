use super::*;

use anyhow::{anyhow, Result};

impl Client {
    pub fn send_tracker_request(&mut self) -> Result<()> {
        let params = TrackerRequestParameters::from_torrent(&self.torrent)?;
        let announce = String::from_utf8(self.torrent.announce.clone())?;
        let final_url = format!("{}?{}", announce, params.as_url_params());

        println!("{}", final_url);
        let response = reqwest::blocking::get(final_url)?.bytes()?.to_vec();
        let tracker_response = TrackerResponse::from_bytes(response)?;

        self.last_response = Some(tracker_response);

        Ok(())
    }
}
