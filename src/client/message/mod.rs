use anyhow::{anyhow, Result};

/// Message used to communicate with peers
#[derive(Debug, PartialEq, Eq)]
pub struct Message {
    pub length: u32,
    pub message_id: Option<u8>,
    pub payload: Option<Vec<u8>>,
}

impl Message {
    /// Parse bytes into a message
    /// TODO: Return a Result instead of panicking
    pub fn from_bytes(data: Vec<u8>) -> Message {
        let length: u32 = u32::from_be_bytes(*array_ref![data, 0, 4]);

        if length == 0 {
            return Message::new_keep_alive();
        }

        let message_id: u8 = u8::from_be_bytes(*array_ref![data, 4, 1]);
        let payload = data
            .get(5..5 + (length as usize - 1))
            .expect("missing payload!");

        match message_id {
            0 => Message::new_choke(),
            1 => Message::new_unchoke(),
            2 => Message::new_interested(),
            3 => Message::new_not_interested(),
            4 => Message::new_have(u32::from_be_bytes(*array_ref![payload, 0, 4])),
            5 => Message::new_bitfield(payload.to_vec()),
            6 => Message::new_request(
                u32::from_be_bytes(*array_ref![payload, 0, 4]),
                u32::from_be_bytes(*array_ref![payload, 4, 4]),
                u32::from_be_bytes(*array_ref![payload, 8, 4]),
            ),
            7 => Message::new_piece(
                u32::from_be_bytes(*array_ref![payload, 0, 4]),
                u32::from_be_bytes(*array_ref![payload, 4, 4]),
                payload[8..].to_vec(),
            ),
            8 => Message::new_cancel(
                u32::from_be_bytes(*array_ref![payload, 0, 4]),
                u32::from_be_bytes(*array_ref![payload, 4, 4]),
                u32::from_be_bytes(*array_ref![payload, 8, 4]),
            ),
            9 => Message::new_port(u16::from_be_bytes(*array_ref![payload, 0, 2])),
            _ => panic!("Unexpected message id"),
        }
    }

    /// Get byte representation of a message
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut out = vec![];

        out.append(&mut self.length.to_be_bytes().to_vec());

        if self.message_id.is_some() {
            out.append(&mut self.message_id.unwrap().to_be_bytes().to_vec());
        }

        if self.payload.is_some() {
            out.append(&mut self.payload.to_owned().unwrap());
        }

        out
    }

    /// Construct a "keep-alive" message
    pub fn new_keep_alive() -> Message {
        Message {
            length: 0,
            message_id: None,
            payload: None,
        }
    }

    /// Construct a "choke" message
    pub fn new_choke() -> Message {
        Message {
            length: 1,
            message_id: Some(0),
            payload: None,
        }
    }

    /// Construct a "unchoke" message
    pub fn new_unchoke() -> Message {
        Message {
            length: 1,
            message_id: Some(1),
            payload: None,
        }
    }

    /// Construct a "interested" message
    pub fn new_interested() -> Message {
        Message {
            length: 1,
            message_id: Some(2),
            payload: None,
        }
    }

    /// Construct a "not_interested" message
    pub fn new_not_interested() -> Message {
        Message {
            length: 1,
            message_id: Some(3),
            payload: None,
        }
    }

    /// Construct a "have" message
    /// TODO: Is u32 the right type?
    pub fn new_have(piece_index: u32) -> Message {
        Message {
            length: 5,
            message_id: Some(4),
            payload: Some(piece_index.to_be_bytes().to_vec()),
        }
    }

    /// Construct a "bitfield" message
    pub fn new_bitfield(bitfield: Vec<u8>) -> Message {
        Message {
            length: 1 + bitfield.len() as u32,
            message_id: Some(5),
            payload: Some(bitfield),
        }
    }

    /// Construct a "request" message
    pub fn new_request(index: u32, begin: u32, length: u32) -> Message {
        let mut buf = vec![];
        buf.append(&mut index.to_be_bytes().to_vec());
        buf.append(&mut begin.to_be_bytes().to_vec());
        buf.append(&mut length.to_be_bytes().to_vec());

        Message {
            length: 13,
            message_id: Some(6),
            payload: Some(buf),
        }
    }

    /// Construct a "piece" message
    pub fn new_piece(index: u32, begin: u32, mut block: Vec<u8>) -> Message {
        let mut buf = vec![];
        buf.append(&mut index.to_be_bytes().to_vec());
        buf.append(&mut begin.to_be_bytes().to_vec());
        buf.append(&mut block);

        Message {
            length: 9 + block.len() as u32,
            message_id: Some(7),
            payload: Some(buf),
        }
    }

    /// Construct a "cancel" message
    pub fn new_cancel(index: u32, begin: u32, length: u32) -> Message {
        let mut buf = vec![];
        buf.append(&mut index.to_be_bytes().to_vec());
        buf.append(&mut begin.to_be_bytes().to_vec());
        buf.append(&mut length.to_be_bytes().to_vec());

        Message {
            length: 13,
            message_id: Some(8),
            payload: Some(buf),
        }
    }

    /// Construct a "port" message
    pub fn new_port(port: u16) -> Message {
        Message {
            length: 3,
            message_id: Some(9),
            payload: Some(port.to_be_bytes().to_vec()),
        }
    }
}
