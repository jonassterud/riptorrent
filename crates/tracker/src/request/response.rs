use anyhow::{anyhow, Result};
use arrayref::array_ref;
use std::net::{IpAddr, Ipv4Addr};
use std::collections::BTreeMap;
use bcode::map_get;

#[derive(Debug)]
pub struct Response {
    pub failure_reason: Option<String>,
    pub warning_message: Option<String>,
    pub interval: Option<i64>,
    pub min_interval: Option<i64>,
    pub tracker_id: Option<String>,
    pub complete: Option<i64>,
    pub incomplete: Option<i64>,
    pub peers: Vec<Peer>,
}

#[derive(Debug)]
pub struct Peer {
    pub peer_id: Option<String>,
    pub ip: Option<IpAddr>,
    pub port: Option<u16>,
}

impl Response {
     /// Converts a vector of bytes to a `Response`.
    ///
    /// # Arguments
    ///
    /// * `vec` - byte vector.
    pub fn from_bytes(vec: Vec<u8>) -> Result<Response> {
        let main_map: BTreeMap<Vec<u8>, bcode::Value> = bcode::decode(&vec, &mut 0)?.try_into()?;

        let failure_reason: Option<String> = Some(map_get(&main_map, "failure reason")?.try_into()?);
        let warning_message: Option<String> = Some(map_get(&main_map, "warning message")?.try_into()?);
        let interval: Option<i64> = Some(map_get(&main_map, "interval")?.try_into()?);
        let min_interval: Option<i64> = Some(map_get(&main_map, "min interval")?.try_into()?);
        let tracker_id: Option<String> = Some(map_get(&main_map, "tracker id")?.try_into()?);
        let complete: Option<i64> = Some(map_get(&main_map, "complete")?.try_into()?);
        let incomplete: Option<i64> = Some(map_get(&main_map, "incomplete")?.try_into()?);

        let peers = match map_get(&main_map, "peers")? {
            bcode::Value::List(peers_list) => {
                let mut out = vec![];

                for peer_map in peers_list {
                    let peer_map: BTreeMap<Vec<u8>, bcode::Value> = peer_map.try_into()?;
                    
                    let peer_id: String = map_get(&peer_map, "peer id")?.try_into()?;
                    let ip: String = map_get(&peer_map, "ip")?.try_into()?;
                    let port: i64 = map_get(&peer_map, "port")?.try_into()?;

                    out.push(Peer {
                        peer_id: Some(peer_id),
                        ip: Some(ip.parse()?),
                        port: Some(port as u16),
                    });
                }
                
                Ok(out)
            },
            bcode::Value::ByteString(peers_bytes) => {
                let mut out = vec![];

                for peer_chunk in peers_bytes.chunks_exact(6) {
                    let ip = array_ref![peer_chunk, 0, 4];
                    let port = u16::from_be_bytes(*array_ref![peer_chunk, 4, 2]);

                    out.push(Peer {
                        peer_id: None,
                        ip: Some(IpAddr::V4(Ipv4Addr::new(ip[0], ip[1], ip[2], ip[3]))),
                        port: Some(port),
                    })
                }
                
                Ok(out)
            },
            _ => Err(anyhow!("Unsupported peers model")),
        }?;
        

        Ok(Response {
            failure_reason,
            warning_message,
            interval,
            min_interval,
            tracker_id,
            complete,
            incomplete,
            peers,
        })
    }
}
