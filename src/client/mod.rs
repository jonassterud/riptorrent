/// Client struct
#[derive(Debug)]
pub struct Client {
    pub torrents: Vec<Torrent>,
    pub request_parameters: TrackerRequestParameters,
    pub last_response: TrackerResponse,
}

/// Tracker request parameters
#[derive(Debug)]
pub struct TrackerRequestParameters {
    pub info_hash: String,
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