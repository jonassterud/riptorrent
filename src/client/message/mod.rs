/// Message used to communicate with peers
#[derive(Debug)]
pub struct Message {
    pub length: u32,
    pub message_id: Option<u8>,
    pub payload: Option<Vec<u8>>,
}

impl Message {
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
