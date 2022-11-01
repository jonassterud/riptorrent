use super::Peer;

use anyhow::{anyhow, Result};

impl Peer {
    /// Handshake with the peer.
    ///
    /// # Arguments
    ///
    /// * `stream` - `TcpStream`.
    /// * `info_hash` - info hash of the torrent.
    /// * `peer_id` - this clients peer id.
    pub async fn handshake(&self, info_hash: &mut Vec<u8>, id: &mut Vec<u8>) -> Result<()> {
        self.send_handshake(info_hash, id).await?;
        self.read_handshake().await?;

        Ok(())
    }

    /// Send a handshake to the stream.
    ///
    /// # Arguments
    ///
    /// * `stream` - `TcpStream`.
    /// * `info_hash` - info hash of the torrent.
    /// * `peer_id` - this clients peer id.
    pub async fn send_handshake(&self, info_hash: &mut Vec<u8>, id: &mut Vec<u8>) -> Result<()> {
        let mut handshake = vec![];
        handshake.push(19_u8);
        handshake.append(&mut b"BitTorrent protocol".to_vec());
        handshake.append(&mut vec![0; 8]);
        handshake.append(info_hash);
        handshake.append(id);

        self.send_data(handshake).await?;

        Ok(())
    }

    /// Read a handshake from the stream.
    ///
    /// # Arguments
    ///
    /// * `stream` - `TcpStream`.
    pub async fn read_handshake(&self) -> Result<Vec<u8>> {
        let mut pstrlen = vec![0; 1];

        self.read_data(&mut pstrlen).await?;

        let buf_length = (49 - 1)
            + *pstrlen
                .first()
                .ok_or_else(|| anyhow!("Failed reading \"pstrlen\""))? as usize;
        let mut buf = vec![0; buf_length];

        self.read_data(&mut buf).await?;

        let mut out = pstrlen.clone();
        out.append(&mut buf);

        Ok(out)
    }
}
