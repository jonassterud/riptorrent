use super::*;
use message::Message;

use anyhow::{anyhow, Result};

use std::io::prelude::*;
use std::net::{SocketAddr, TcpStream};

/// Struct to represent a peer
#[derive(Debug)]
pub struct Peer {
    pub peer_id: Option<Vec<u8>>,
    pub ip: [u8; 4],
    pub port: u16,
    pub stream: Option<TcpStream>,
}

impl Peer {
    pub fn connect(&mut self) -> Result<()> {
        self.stream = Some(TcpStream::connect(SocketAddr::from((self.ip, self.port)))?);

        Ok(())
    }

    /// Send data
    pub fn send(&mut self, data: Vec<u8>) -> Result<()> {
        if let Some(stream) = self.stream.as_mut() {
            stream.write_all(&data)?;
        }

        Ok(())
    }

    /// Receive data
    pub fn receive(&mut self, buf: &mut [u8]) -> Result<()> {
        if let Some(stream) = self.stream.as_mut() {
            stream.read_exact(buf)?;
        }

        Ok(())
    }
}
