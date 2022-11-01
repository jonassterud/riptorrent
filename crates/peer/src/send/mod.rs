use super::Peer;

use anyhow::{anyhow, Result};
use async_std::io::WriteExt;
use message::Message;

impl Peer {
    /// Send data to stream.
    ///
    /// # Arguments
    ///
    /// * `bytes` - data to send to peer.
    pub async fn send_data(&self, bytes: Vec<u8>) -> Result<()> {
        self.stream
            .clone()
            .ok_or_else(|| anyhow!("Missing stream"))?
            .lock()
            .await
            .write_all(&bytes)
            .await?;

        Ok(())
    }

    /// Send a `Message` to the stream.
    ///
    /// # Arguments
    ///
    /// * `message` - message to send.
    pub async fn send_message(&self, message: Message) -> Result<()> {
        println!("Sent: {}", message.get_name());
        let bytes = message.into_bytes();
        self.send_data(bytes).await?;

        Ok(())
    }
}
