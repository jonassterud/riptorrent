mod cli;

use anyhow::{anyhow, Result};
use arrayref::array_ref;
use async_std::sync::{Arc, Mutex};
use cli::*;
use message::Message;
use torrent::Torrent;

#[async_std::main]
async fn main() -> Result<()> {
    let args = cli::Args::parse();

    if let Ok(bytes) = std::fs::read(args.path) {
        let torrent = Torrent::from_bytes(bytes).await?;

        let peer_id = b"-qBhj010488887635243".to_vec();

        let tracker_request = tracker::Request::from_torrent(&torrent, &peer_id).await;
        let tracker_resp = tracker_request.send_request().await?;

        println!("Peer amount: {}", tracker_resp.peers.len());

        let pieces: Arc<Mutex<Vec<Vec<u8>>>> =
            Arc::new(Mutex::new(vec![
                vec![0; torrent.get_piece_length() as usize];
                torrent.get_piece_amount()
            ]));
        for peer in tracker_resp.peers {
            let info_hash = torrent.info_hash.clone();
            let piece_amount = torrent.get_piece_amount();
            let peer_id = peer_id.clone();
            let pieces = pieces.clone();

            async_std::task::spawn(async move {
                let mut am_choking = true;
                let mut am_interested = false;
                let mut peer_choking = true;
                let mut peer_interested = false;

                println!("Opening stream to : {:?}", peer.ip);
                // Open stream
                let mut stream = protocol::open_stream(
                    peer.ip.ok_or_else(|| anyhow!("Missing ip"))?,
                    peer.port.ok_or_else(|| anyhow!("Missing port"))?,
                )
                .await?;

                println!("Opened stream");

                // Handshake
                protocol::send_handshake(&mut stream, &info_hash, &peer_id).await?;
                let peer_handshake = protocol::read_handshake(&mut stream).await?;
                println!("Received handshake!");
                println!("Handshake: {:?}", peer_handshake);

                // Recieve/send bitfield
                protocol::send_message(&mut stream, Message::new_bitfield(vec![0; piece_amount]))
                    .await?;
                let mut peer_bitfield: Option<Vec<u8>> = None;

                loop {
                    let message_from_peer = protocol::read_message(&mut stream).await?;
                    match message_from_peer.message_id {
                        Some(0) => peer_choking = true,
                        Some(1) => peer_choking = false,
                        Some(2) => peer_interested = true,
                        Some(3) => peer_interested = false,
                        Some(4) => {
                            println!("Peer has a piece, updating bitfield...");
                            if let Some(bitfield) = &mut peer_bitfield {
                                // should piece index be bigger than u32?
                                let payload = message_from_peer
                                    .payload
                                    .ok_or_else(|| anyhow!("Missing payload"))?;
                                let piece_index = u32::from_be_bytes(*array_ref![payload, 0, 4]);

                                // I think this works.. TODO
                                let bitfield_y = piece_index as usize / 8;
                                let bitfield_x = piece_index as usize % 8;
                                *bitfield.get_mut(bitfield_y).unwrap() =
                                    bitfield.get(bitfield_y).unwrap() ^ (1 << (7 - bitfield_x));
                            } else {
                                println!("Peer hasnt sent bitfield!");
                                panic!()
                            }
                        }
                        Some(5) => {
                            println!("Recieved bitfield!");
                            peer_bitfield = Some(
                                message_from_peer
                                    .payload
                                    .ok_or_else(|| anyhow!("Missing payload"))?,
                            );
                        }
                        Some(6) => {
                            println!("Peer requested piece!");
                        }
                        Some(7) => {
                            let payload = message_from_peer
                                .payload
                                .ok_or_else(|| anyhow!("Missing payload"))?;

                            let piece_index = u32::from_be_bytes(*array_ref![payload, 0, 4]);
                            let piece_begin =
                                u32::from_be_bytes(*array_ref![payload, 4, 4]) as usize;
                            let piece_data = payload
                                .get(8..)
                                .ok_or_else(|| anyhow!("Missing piece data"))?;
                            let piece_end = piece_begin + (piece_begin + piece_data.len());

                            let piece_ref = &mut *pieces.lock().await;
                            let row_ref = piece_ref
                                .get_mut(piece_index as usize)
                                .ok_or_else(|| anyhow!("Pieces structured incorrectly"))?;

                            *row_ref = row_ref
                                .splice(piece_begin..piece_end, piece_data.to_owned())
                                .collect();
                        }
                        Some(8) => {
                            println!("Peer sent cancel");
                        }
                        Some(9) => {
                            println!("Peer sent port");
                        }
                        _ => {}
                    }
                }

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
