use anyhow::Result;
use async_std::io::ReadExt;
use async_std::net::TcpStream;
use async_std::sync::{Arc, Mutex};
use message::Message;

/// Read from the stream into the buffer, and return the number of bytes read.
///
/// # Arguments
///
/// * `stream` - TcpStream.
/// * `buf` - buffer to read to.
pub async fn read_data(stream: Arc<Mutex<TcpStream>>, buf: &mut [u8]) -> Result<usize> {
    let bytes_read = stream.lock().await.read(buf).await?;

    Ok(bytes_read)
}

/// Read a `Message` from the stream.
///
/// # Arguments
///
/// * `stream` - `TcpStream`.
pub async fn read_message(stream: Arc<Mutex<TcpStream>>) -> Result<Message> {
    let mut entire_byte_message = vec![];

    let mut length_buf = [0_u8; 4];
    read_data(stream.clone(), &mut length_buf).await?;
    entire_byte_message.append(&mut length_buf.to_vec());

    let length = u32::from_be_bytes(length_buf) as usize;
    let mut buf = vec![0_u8; length];
    read_data(stream, &mut buf).await?;
    entire_byte_message.append(&mut buf);

    Message::from_bytes(entire_byte_message)
}
