mod cli;

use anyhow::{anyhow, Result};
use arrayref::array_ref;
use async_std::sync::{Arc, Mutex};
use builder::{Block, Builder};
use cli::*;
use message::Message;
use torrent::Torrent;

// TODO list:
//
// * IPv6 not working? - find workaround for networks that don't have 6rd or similar.
// * Generate a peer id.
// * Create piece download strategy.

async fn sleep(secs: u64) {
    async_std::task::sleep(std::time::Duration::from_secs(secs)).await;
}

#[async_std::main]
async fn main() -> Result<()> {
    let args = cli::Args::parse();

    if let Ok(bytes) = std::fs::read(args.path) {
        // Open torrent and get information from tracker.
        let torrent = Torrent::from_bytes(bytes).await?;
        let peer_id = b"-qBhj010488887635243".to_vec();
        let tracker_resp = tracker::Request::from_torrent(&torrent, &peer_id)
            .await
            .send_request()
            .await?;

        // Create builder
        let builder = Arc::new(Mutex::new(Builder::new(
            torrent.get_piece_amount(),
            torrent.get_piece_length() as usize,
            u32::pow(2, 14) as usize,
        )));

        // Print info.
        println!(
            "Peer amount: {}\nPiece amount: {}\nPiece length: {}",
            tracker_resp.peers.len(),
            torrent.get_piece_amount(),
            torrent.get_piece_length()
        );

        // Loop trough peers.
        for (i, mut peer) in tracker_resp.peers.into_iter().enumerate() {
            if i > 15 {
                break;
            }

            // Clone values.
            let mut info_hash = torrent.info_hash.clone();
            let mut id = peer_id.clone();
            let piece_amount = torrent.get_piece_amount();
            let builder = builder.clone();

            // Spawn an async task.
            async_std::task::spawn(async move {
                peer.setup(&mut info_hash, &mut id, piece_amount).await?;
                println!("Ready with {:?}", peer.ip);
                peer.start(builder).await?;

                /*
                // Communication loop with peer.
                loop {
                    // TODO: Send "request" to peer.
                    if am_interested && !peer_choking {
                        protocol::send_message(
                            stream.clone(),
                            Message::new_request(0, 0, u32::pow(2, 14)),
                        )
                        .await?;
                        println!("Sent: \"request\" to peer.");
                        am_interested = false;
                    }

                    // Read message
                    let recieved_message = protocol::read_message(stream.clone()).await?;
                    if recieved_message.get_id().is_some() {
                        println!("Recieved: {}", recieved_message.get_name());
                    }

                    match recieved_message {
                        Message::KeepAlive => {}
                        Message::Choke(_) => peer_choking = true,
                        Message::Unchoke(_) => peer_choking = false,
                        Message::Interested(_) => peer_interested = true,
                        Message::NotInterested(_) => peer_interested = false,
                        Message::Have((_, payload)) => {
                            let piece_index = u32::from_be_bytes(*array_ref![payload, 0, 4]);

                            // TODO: Check that this actually works.
                            let bitfield_y = piece_index as usize / 8;
                            let bitfield_x = piece_index as usize % 8;
                            *peer_bitfield.get_mut(bitfield_y).unwrap() =
                                peer_bitfield.get(bitfield_y).unwrap() ^ (1 << (7 - bitfield_x));
                        }
                        Message::Bitfield((_, payload)) => {
                            peer_bitfield = payload;
                        }
                        Message::Request((_, payload)) => {
                            let piece_index = u32::from_be_bytes(*array_ref![payload, 0, 4]);
                            let piece_begin =
                                u32::from_be_bytes(*array_ref![payload, 4, 4]) as usize;
                            let piece_length =
                                u32::from_be_bytes(*array_ref![payload, 4, 4]) as usize;

                            if peer_interested && !am_choking {
                                let piece_block = builder
                                    .lock()
                                    .await
                                    .get_finished_block(
                                        piece_index as usize,
                                        piece_begin,
                                        piece_length,
                                    )?
                                    .data;

                                // Send "piece" to peer.
                                protocol::send_message(
                                    stream.clone(),
                                    Message::new_piece(
                                        piece_index,
                                        piece_begin as u32,
                                        piece_block,
                                    ),
                                )
                                .await?;
                                // println!("Sent: \"piece\" to peer.");
                            }
                        }
                        Message::Piece((_, payload)) => {
                            let piece_index = u32::from_be_bytes(*array_ref![payload, 0, 4]);
                            let piece_begin =
                                u32::from_be_bytes(*array_ref![payload, 4, 4]) as usize;
                            let piece_data = payload
                                .get(8..)
                                .ok_or_else(|| anyhow!("Missing piece data"))?;

                            let block = Block {
                                index: piece_index as usize,
                                begin: piece_begin as usize,
                                data: piece_data.to_vec(),
                            };

                            builder.lock().await.add_finished_block(block)?;

                            am_interested = true;
                        }
                        Message::Cancel(_) => {
                            // ...
                        }
                        Message::Port(_) => {
                            // ...
                        }
                    }

                    sleep(3).await;
                }

                */

                Ok::<(), anyhow::Error>(())
            });
        }

        // wait
        std::io::stdin().read_line(&mut String::new()).unwrap();

        Ok(())
    } else {
        Err(anyhow!("Failed reading torrent file"))
    }
}

// Send "interested" to peer.
//protocol::send_message(stream.clone(), Message::new_interested()).await?;
//am_interested = true;
//println!("Sent: \"interested\" to peer.");
