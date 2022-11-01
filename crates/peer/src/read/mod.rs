use super::Peer;

use anyhow::{anyhow, Result};
use async_std::io::ReadExt;
use message::Message;

impl Peer {
    /// Read from the stream into the buffer, and return the number of bytes read.
    ///
    /// # Arguments
    ///
    /// * `buf` - buffer to read to.
    pub async fn read_data(&self, buf: &mut [u8]) -> Result<usize> {
        let bytes_read = self
            .stream
            .clone()
            .ok_or_else(|| anyhow!("Missing stream"))?
            .lock()
            .await
            .read(buf)
            .await?;

        Ok(bytes_read)
    }

    /// Read a `Message` from the stream.
    pub async fn read_message(&self) -> Result<Message> {
        let mut entire_byte_message = vec![];

        let mut length_buf = [0_u8; 4];
        self.read_data(&mut length_buf).await?;
        entire_byte_message.append(&mut length_buf.to_vec());

        let length = u32::from_be_bytes(length_buf) as usize;
        let mut buf = vec![0_u8; length];
        self.read_data(&mut buf).await?;
        entire_byte_message.append(&mut buf);

        Message::from_bytes(entire_byte_message)
    }
}
