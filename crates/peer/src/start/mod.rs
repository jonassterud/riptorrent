mod handle_response;

use super::Peer;
use anyhow::{anyhow, Result};
use arrayref::array_ref;
use async_std::sync::{Arc, Mutex};
use builder::{Block, Builder};
use handle_response::handle_response;
use message::Message;

impl Peer {
    /// Start a communication loop with the peer.
    /// Only exists if connection is no longer interesting.
    pub async fn start(&mut self, builder: Arc<Mutex<Builder>>) -> Result<()> {
        loop {
            let recieved_message = self.read_message().await?;
            handle_response(recieved_message, self, builder.clone()).await?;

            if self.bitfield.is_some() {
                //builder.lock().await.take_random_missing_block()?;
            }
        }

        Ok(())
    }
}
