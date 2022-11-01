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
        let request_limit = 10;
        let mut wanted_blocks: Vec<Block> = vec![];

        loop {
            let recieved_message = self.read_message().await?;

            if recieved_message.get_name() != "Keep alive" {
                println!("Got: {}", recieved_message.get_name());
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
                        //     break;
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

                    if let Some((index, _)) =
                        wanted_blocks.iter().enumerate().find(|x| x.1.eq(&block))
                    {
                        wanted_blocks.swap_remove(index);
                    }

                    builder.lock().await.add_finished_block(block)?;

                    //let finished = builder.lock().await.finished.len();
                    //let total = finished + builder.lock().await.missing.len();

                    println!("got piece");
                }
                Message::Cancel(_) => {
                    todo!()
                }
                Message::Port(_) => {
                    // todo!()
                }
            };

            if let Some(bitfield) = &self.bitfield {
                if wanted_blocks.len() < request_limit {
                    if let Ok(t) = builder.lock().await.take_missing_relevant_block(bitfield) {
                        wanted_blocks.push(t);
                    }

                    if !self.am_interested {
                        self.send_message(Message::new_interested()).await?;
                        self.send_message(Message::new_unchoke()).await?;
                        self.am_interested = true;
                    }
                }

                if !wanted_blocks.is_empty() && !self.peer_choking {
                    self.send_message(Message::new_request(
                        wanted_blocks.get(0).unwrap().index as u32,
                        wanted_blocks.get(0).unwrap().begin as u32,
                        wanted_blocks.get(0).unwrap().data.len() as u32,
                    ))
                    .await?;

                    // TODO: Return block to pool if not found!
                    // TODO: Send "have" when recieving a piece!
                } else {
                    async_std::task::sleep(std::time::Duration::from_secs(3)).await;
                    self.send_message(Message::new_keep_alive()).await?;
                }
            }
        }
    }
}
