use super::*;

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
        Ok(TrackerRequestParameters {
            info_hash: torrent.info_hash.clone(),
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
