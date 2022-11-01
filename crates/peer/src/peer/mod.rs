use anyhow::Result;
use async_std::net::{IpAddr, SocketAddr, TcpStream};
use async_std::sync::{Arc, Mutex};

/// Struct representing a peer from a tracker response.
#[derive(Debug, Clone)]
pub struct Peer {
    pub id: Option<Vec<u8>>,
    pub ip: IpAddr,
    pub port: u16,

    pub stream: Option<Arc<Mutex<TcpStream>>>,
    pub am_choking: bool,
    pub am_interested: bool,
    pub peer_choking: bool,
    pub peer_interested: bool,
    pub bitfield: Option<Vec<u8>>,
}

impl Peer {
    pub fn new(id: Option<Vec<u8>>, ip: IpAddr, port: u16) -> Peer {
        Peer {
            id,
            ip,
            port,
            stream: None,
            am_choking: true,
            am_interested: false,
            peer_choking: true,
            peer_interested: false,
            bitfield: None,
        }
    }

    /// Open a `TcpStream`.
    pub async fn open_stream(&mut self) -> Result<()> {
        self.stream = Some(Arc::new(Mutex::new(
            TcpStream::connect(SocketAddr::new(self.ip, self.port)).await?,
        )));

        Ok(())
    }
}
