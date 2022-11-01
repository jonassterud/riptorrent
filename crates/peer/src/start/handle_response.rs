use super::Peer;
use anyhow::{anyhow, Result};
use arrayref::array_ref;
use async_std::sync::{Arc, Mutex};
use builder::{Block, Builder};
use message::Message;

pub async fn handle_response(
    message: Message,
    peer: &mut Peer,
    builder: Arc<Mutex<Builder>>,
) -> Result<()> {
    match message {
        Message::KeepAlive => {}
        Message::Choke(_) => peer.peer_choking = true,
        Message::Unchoke(_) => peer.peer_choking = false,
        Message::Interested(_) => peer.peer_interested = true,
        Message::NotInterested(_) => peer.peer_interested = false,
        Message::Have((_, payload)) => {
            let piece_index = u32::from_be_bytes(*array_ref![payload, 0, 4]);

            // TODO: Check that this actually works.
            let bitfield_y = piece_index as usize / 8;
            let bitfield_x = piece_index as usize % 8;
            let bitfield = peer
                .bitfield
                .as_mut()
                .ok_or_else(|| anyhow!("Missing bitfield"))?;
            *bitfield.get_mut(bitfield_y).unwrap() =
                bitfield.get(bitfield_y).unwrap() ^ (1 << (7 - bitfield_x));
        }
        Message::Bitfield((_, payload)) => peer.bitfield = Some(payload),
        Message::Request((_, payload)) => {
            if !peer.peer_interested || peer.am_choking {
                return Ok(());
            }

            let piece_index = u32::from_be_bytes(*array_ref![payload, 0, 4]);
            let piece_begin = u32::from_be_bytes(*array_ref![payload, 4, 4]) as usize;
            let piece_length = u32::from_be_bytes(*array_ref![payload, 4, 4]) as usize;

            let piece_block = builder
                .lock()
                .await
                .get_finished_block(piece_index as usize, piece_begin, piece_length)?
                .data;

            peer.send_message(Message::new_piece(
                piece_index,
                piece_begin as u32,
                piece_block,
            ))
            .await?;
        }
        Message::Piece((_, payload)) => {
            if !peer.am_interested || peer.peer_choking {
                return Ok(());
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
        }
        Message::Cancel(_) => {
            todo!()
        }
        Message::Port(_) => {
            todo!()
        }
    };

    Ok(())
}
