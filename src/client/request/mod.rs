use super::*;

use rand::prelude::*;

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
    pub event: String,
}

impl TrackerRequestParameters {
    /// Construct a new `TrackerRequestParameters` from a torrent
    ///
    /// # Arguments
    ///
    /// * `torrent` - a reference to a `Torrent` struct
    // TODO: Replace placeholder data
    pub fn from_torrent(torrent: &Torrent) -> Result<TrackerRequestParameters> {
        let peer_id: String = ['_'; 20]
            .map(|_| (thread_rng().gen_range(32..=126) as u8) as char)
            .iter()
            .collect();

        Ok(TrackerRequestParameters {
            info_hash: torrent.info_hash.clone(),
            peer_id,
            port: "6881".to_string(),
            uploaded: "0".to_string(),
            downloaded: "0".to_string(),
            left: torrent.get_left().to_string(),
            compact: "1".to_string(),
            event: "started".to_string(),
        })
    }

    /// Returns the parameters as a string to add it to a URL
    pub fn as_url_params(&self) -> String {
        format!(
            "info_hash={}&peer_id={}&port={}&uploaded={}&downloaded={}&left={}&compact={}&event={}",
            urlencoding::encode_binary(&self.info_hash),
            urlencoding::encode(&self.peer_id),
            urlencoding::encode(&self.port),
            urlencoding::encode(&self.uploaded),
            urlencoding::encode(&self.downloaded),
            urlencoding::encode(&self.left),
            urlencoding::encode(&self.compact),
            urlencoding::encode(&self.event),
        )
    }
}
