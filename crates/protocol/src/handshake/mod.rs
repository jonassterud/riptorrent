use super::read_data;

use anyhow::{anyhow, Result};
use async_std::io::WriteExt;
use async_std::net::TcpStream;
use async_std::sync::{Arc, Mutex};

/// Handshake with the peer.
///
/// # Arguments
///
/// * `stream` - `TcpStream`.
/// * `info_hash` - info hash of the torrent.
/// * `peer_id` - this clients peer id.
pub async fn handshake(
    stream: Arc<Mutex<TcpStream>>,
    info_hash: &[u8],
    peer_id: &[u8],
) -> Result<()> {
    futures::try_join!(
        send_handshake(stream.clone(), info_hash, peer_id),
        read_handshake(stream)
    )?;

    Ok(())
}

/// Send a handshake to the stream.
///
/// # Arguments
///
/// * `stream` - `TcpStream`.
/// * `info_hash` - info hash of the torrent.
/// * `peer_id` - this clients peer id.
pub async fn send_handshake(
    stream: Arc<Mutex<TcpStream>>,
    info_hash: &[u8],
    peer_id: &[u8],
) -> Result<()> {
    let mut handshake = vec![];
    handshake.push(19_u8);
    handshake.append(&mut b"BitTorrent protocol".to_vec());
    handshake.append(&mut vec![0; 8]);
    handshake.append(&mut info_hash.to_vec());
    handshake.append(&mut peer_id.to_vec());

    stream.lock().await.write_all(&handshake).await?;

    Ok(())
}

/// Read a handshake from the stream.
///
/// # Arguments
///
/// * `stream` - `TcpStream`.
pub async fn read_handshake(stream: Arc<Mutex<TcpStream>>) -> Result<Vec<u8>> {
    let mut pstrlen = vec![0; 1];

    read_data(stream.clone(), &mut pstrlen).await?;

    let buf_length = (49 - 1)
        + *pstrlen
            .first()
            .ok_or_else(|| anyhow!("Failed reading \"pstrlen\""))? as usize;
    let mut buf = vec![0; buf_length];

    read_data(stream, &mut buf).await?;

    let mut out = pstrlen.clone();
    out.append(&mut buf);

    Ok(out)
}
