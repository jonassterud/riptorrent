use super::*;

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
