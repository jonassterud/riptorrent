use super::Peer;
use anyhow::{anyhow, Result};
use arrayref::array_ref;
use async_std::sync::{Arc, Mutex};
use builder::{Block, Builder};
use message::Message;

impl Peer {
    /// Start a communication loop with the peer.
    /// Only exists if connection is no longer interesting.
    pub async fn start(&mut self, builder: Arc<Mutex<Builder>>) -> Result<()> {
        let mut wanted_block = None;

        loop {
            let recieved_message = self.read_message().await?;
            if recieved_message.get_name() != "Keep alive" && recieved_message.get_name() != "Unchoke" {
                println!("Got {}", recieved_message.get_name());
            }

            match recieved_message {
                Message::KeepAlive => {}
                Message::Choke(_) => self.peer_choking = true,
                Message::Unchoke(_) => self.peer_choking = false,
                Message::Interested(_) => self.peer_interested = true,
                Message::NotInterested(_) => self.peer_interested = false,
                Message::Have((_, payload)) => {
                    let piece_index = u32::from_be_bytes(*array_ref![payload, 0, 4]);

                    // TODO: Check that this actually works.
                    let bitfield_y = piece_index as usize / 8;
                    let bitfield_x = piece_index as usize % 8;
                    let bitfield = self
                        .bitfield
                        .as_mut()
                        .ok_or_else(|| anyhow!("Missing bitfield"))?;
                    *bitfield.get_mut(bitfield_y).unwrap() =
                        bitfield.get(bitfield_y).unwrap() ^ (1 << (7 - bitfield_x));
                }
                Message::Bitfield((_, payload)) => self.bitfield = Some(payload),
                Message::Request((_, payload)) => {
                    if !self.peer_interested || self.am_choking {
                        break;
                    }

                    let piece_index = u32::from_be_bytes(*array_ref![payload, 0, 4]);
                    let piece_begin = u32::from_be_bytes(*array_ref![payload, 4, 4]) as usize;
                    let piece_length = u32::from_be_bytes(*array_ref![payload, 4, 4]) as usize;

                    let piece_block = builder
                        .lock()
                        .await
                        .get_finished_block(piece_index as usize, piece_begin, piece_length)?
                        .data;

                    self.send_message(Message::new_piece(
                        piece_index,
                        piece_begin as u32,
                        piece_block,
                    ))
                    .await?;
                }
                Message::Piece((_, payload)) => {
                    if !self.am_interested || self.peer_choking {
                        break;
                    }

                    let piece_index = u32::from_be_bytes(*array_ref![payload, 0, 4]);
                    let piece_begin = u32::from_be_bytes(*array_ref![payload, 4, 4]) as usize;
                    let piece_data = payload
                        .get(8..)
                        .ok_or_else(|| anyhow!("Missing piece data"))?;

                    let block = Block {
                        index: piece_index as usize,
                        begin: piece_begin as usize,
                        data: piece_data.to_vec(),
                    };

                    builder.lock().await.add_finished_block(block)?;
                    wanted_block = None;
                }
                Message::Cancel(_) => {
                    todo!()
                }
                Message::Port(_) => {
                    todo!()
                }
            };

            if let Some(bitfield) = &self.bitfield {
                if !self.peer_choking && self.am_interested && wanted_block.is_none() {
                    wanted_block = builder
                        .lock()
                        .await
                        .take_missing_relevant_block(bitfield)
                        .ok();

                    self.send_message(Message::new_request(
                        wanted_block.as_ref().unwrap().index as u32,
                        wanted_block.as_ref().unwrap().begin as u32,
                        wanted_block.as_ref().unwrap().data.len() as u32,
                    ))
                    .await?;
                    println!("Sent request");
                    
                    // TODO: Return block to pool if not found!
                } else {
                    self.send_message(Message::new_interested()).await?;
                    self.am_interested = true;
                }
                
            }
        }

        Ok(())
    }
}
