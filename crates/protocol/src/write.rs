use anyhow::Result;
use async_std::io::WriteExt;
use async_std::net::TcpStream;
use async_std::sync::{Arc, Mutex};
use message::Message;

/// Send a `Message` to the stream.
///
/// # Arguments
///
/// * `stream` - `TcpStream`.
/// * `message` - message to send.
pub async fn send_message(stream: Arc<Mutex<TcpStream>>, message: Message) -> Result<()> {
    let bytes = message.into_bytes();
    stream.lock().await.write_all(&bytes).await?;

    Ok(())
}
