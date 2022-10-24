use crate::Message;
use anyhow::{anyhow, Result};
use arrayref::array_ref;

impl Message {
    /// Converts a byte vector to a `Message`.
    ///
    /// # Arguments
    ///
    /// * `vec` - byte vector.
    pub fn from_bytes(vec: Vec<u8>) -> Result<Message> {
        let length: u32 = u32::from_be_bytes(*array_ref![vec, 0, 4]);

        if length == 0 {
            return Ok(Message::new_keep_alive());
        }

        let message_id: u8 = u8::from_be_bytes(*array_ref![vec, 4, 1]);
        let payload = vec
            .get(5..5 + (length as usize - 1))
            .ok_or_else(|| anyhow!("Missing message payload"))?;

        match message_id {
            0 => Ok(Message::new_choke()),
            1 => Ok(Message::new_unchoke()),
            2 => Ok(Message::new_interested()),
            3 => Ok(Message::new_not_interested()),
            4 => Ok(Message::new_have(u32::from_be_bytes(*array_ref![
                payload, 0, 4
            ]))),
            5 => Ok(Message::new_bitfield(payload.to_vec())),
            6 => Ok(Message::new_request(
                u32::from_be_bytes(*array_ref![payload, 0, 4]),
                u32::from_be_bytes(*array_ref![payload, 4, 4]),
                u32::from_be_bytes(*array_ref![payload, 8, 4]),
            )),
            7 => Ok(Message::new_piece(
                u32::from_be_bytes(*array_ref![payload, 0, 4]),
                u32::from_be_bytes(*array_ref![payload, 4, 4]),
                payload[8..].to_vec(),
            )),
            8 => Ok(Message::new_cancel(
                u32::from_be_bytes(*array_ref![payload, 0, 4]),
                u32::from_be_bytes(*array_ref![payload, 4, 4]),
                u32::from_be_bytes(*array_ref![payload, 8, 4]),
            )),
            9 => Ok(Message::new_port(u16::from_be_bytes(*array_ref![
                payload, 0, 2
            ]))),
            _ => Err(anyhow!("Unexpected message id")),
        }
    }
}
