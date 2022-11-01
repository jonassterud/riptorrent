use super::Peer;
use message::Message;

use anyhow::Result;

impl Peer {
    /// Setup connection, handshake and send bitfield.
    pub async fn setup(
        &mut self,
        info_hash: &mut Vec<u8>,
        id: &mut Vec<u8>,
        piece_amount: usize,
    ) -> Result<()> {
        self.open_stream().await?;

        self.handshake(info_hash, id).await?;

        self.bitfield = Some(vec![0; piece_amount / 8]);
        let bitfield_message = Message::new_bitfield(self.bitfield.clone().unwrap());
        self.send_message(bitfield_message).await?;

        Ok(())
    }
}
