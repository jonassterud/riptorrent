mod cli;

use anyhow::{anyhow, Result};
use arrayref::array_ref;
use async_std::sync::{Arc, Mutex, RwLock};
use cli::*;
use message::Message;
use torrent::Torrent;

// TODO list:
//
// * IPv6 not working - find workaround for networks that don't have 6rd or similar.
// * Generate a peer id.

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

        // Create pieces vector.
        let pieces: Arc<Mutex<Vec<Vec<u8>>>> =
            Arc::new(Mutex::new(vec![
                vec![0; torrent.get_piece_length() as usize];
                torrent.get_piece_amount()
            ]));

        println!("Peer amount: {}", tracker_resp.peers.len());

        // Loop trough peers.
        for peer in tracker_resp.peers {
            // Clone values.
            let info_hash = torrent.info_hash.clone();
            let piece_amount = torrent.get_piece_amount();
            let peer_id = peer_id.clone();
            let pieces = pieces.clone();

            // Spawn an async task.
            async_std::task::spawn(async move {
                // Set peer state variables.
                let mut am_choking = true;
                let mut am_interested = false;
                let mut peer_choking = true;
                let mut peer_interested = false;
                let mut peer_bitfield = vec![0_u8; piece_amount / 8];
    
                // Open TcpStream to peer
                let stream = protocol::open_stream(
                    peer.ip.ok_or_else(|| anyhow!("Missing ip"))?,
                    peer.port.ok_or_else(|| anyhow!("Missing port"))?,
                )
                .await?; // TODO: Gets stuck on ipv6 addresses.
                println!("Opened stream to: {} -> {}", peer.ip.unwrap(), peer.port.unwrap());

                // Handshake with peer.
                protocol::handshake(stream.clone(), &info_hash, &peer_id).await?;
                println!("Handshaked with {}", peer.ip.unwrap());

                // Send "bitfield" to peer.
                protocol::send_message(
                    stream.clone(),
                    Message::new_bitfield(vec![0; piece_amount / 8]),
                )
                .await?;
                println!("Sent \"bitfield\" to peer.");
            
                // Send "unchoke" to peer.
                protocol::send_message(stream.clone(), Message::new_unchoke()).await?;
                am_choking = false;
                println!("Sent: \"unchoke\" to peer.");

                // Send "interested" to peer.
                protocol::send_message(stream.clone(), Message::new_interested()).await?;
                am_interested = true;
                println!("Sent: \"interested\" to peer.");

                // Communication loop with peer.
                loop {
                    // TODO: Send "request" to peer.
                    if am_interested && !peer_choking {
                        protocol::send_message(stream.clone(), Message::new_request(0, 0, u32::pow(2, 14))).await?;
                        println!("Sent: \"request\" to peer.");
                        am_interested = false;
                    }

                    // Read message
                    let recieved_message = protocol::read_message(stream.clone()).await?;
                    if recieved_message.get_id().is_some() {
                        println!("Recieved: {:?}", recieved_message);
                    }

                    match recieved_message {
                        Message::Choke(_) => peer_choking = true,
                        Message::Unchoke(_) => peer_choking = false,
                        Message::Interested(_) => peer_interested = true,
                        Message::NotInterested(_) => peer_interested = false,
                        Message::Have((_, payload)) => {
                            let piece_index = u32::from_be_bytes(*array_ref![payload, 0, 4]);

                            // TODO: Check that this actually works.
                            let bitfield_y = piece_index as usize / 8;
                            let bitfield_x = piece_index as usize % 8;
                            *peer_bitfield.get_mut(bitfield_y).unwrap() = peer_bitfield.get(bitfield_y).unwrap() ^ (1 << (7 - bitfield_x));
                        },
                        Message::Bitfield((_, payload)) => {
                            peer_bitfield = payload;
                        },
                        Message::Request((_, payload)) => {
                            // ...
                        },
                        Message::Piece((_, payload)) => {
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
                        },
                        Message::Cancel(_) => {
                            // ...
                        },
                        Message::Port(_) => {
                            // ...
                        },
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
