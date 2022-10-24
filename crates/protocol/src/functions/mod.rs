use anyhow::{anyhow, Result};
use async_std::io::{ReadExt, WriteExt};
use async_std::net::{IpAddr, SocketAddr, TcpStream};
use message::Message;

/// Open a `TcpStream`.
pub async fn open_stream(ip: IpAddr, port: u16) -> Result<TcpStream> {
    Ok(TcpStream::connect(SocketAddr::new(ip, port)).await?)
}

/// Read from the stream into the buffer, and return the number of bytes read.
pub async fn read_message(stream: &mut TcpStream, buf: &mut [u8]) -> Result<usize> {
    let bytes_read = stream.read(buf).await?;

    Ok(bytes_read)
}

/// Send a `Message` to the stream.
///
/// # Arguments
///
/// * `stream` - async `TcpStream`.
/// * `message` - message to send.
pub async fn send_message(stream: &mut TcpStream, message: Message) -> Result<()> {
    let bytes = message.into_bytes();
    stream.write_all(&bytes).await?;

    Ok(())
}

/// Send a handshake to the stream.
///
/// # Arguments
///
/// * `stream` - async `TcpStream`.
/// * `info_hash` - info hash of the torrent.
/// * `peer_id` - this clients peer id.
pub async fn send_handshake(
    stream: &mut TcpStream,
    info_hash: &[u8],
    peer_id: &[u8],
) -> Result<()> {
    let mut handshake = vec![];
    handshake.push(19_u8);
    handshake.append(&mut b"BitTorrent protocol".to_vec());
    handshake.append(&mut vec![0; 8]);
    handshake.append(&mut info_hash.to_vec());
    handshake.append(&mut peer_id.to_vec());

    stream.write_all(&handshake).await?;

    Ok(())
}

/// Read a handshake from the stream.
pub async fn read_handshake(stream: &mut TcpStream) -> Result<Vec<u8>> {
    let mut pstrlen = vec![0; 1];

    read_message(stream, &mut pstrlen).await?;

    let buf_length = (49 - 1)
        + *pstrlen
            .first()
            .ok_or_else(|| anyhow!("Failed reading \"pstrlen\""))? as usize;
    let mut buf = vec![0; buf_length];

    read_message(stream, &mut buf).await?;

    let mut out = pstrlen.clone();
    out.append(&mut buf);

    Ok(out)
}
