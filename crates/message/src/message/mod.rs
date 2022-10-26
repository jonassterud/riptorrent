mod from_bytes;
mod into_bytes;

pub type MessageData = (u8, Vec<u8>);

/// A message used to communicate on the BitTorrent network.
#[derive(Debug, PartialEq, Eq)]
pub enum Message {
    KeepAlive,
    Choke(MessageData),
    Unchoke(MessageData),
    Interested(MessageData),
    NotInterested(MessageData),
    Have(MessageData),
    Bitfield(MessageData),
    Request(MessageData),
    Piece(MessageData),
    Cancel(MessageData),
    Port(MessageData),
}

impl Message {
    /// Returns the id of a message, if any.
    pub fn get_id(&self) -> Option<u8> {
        use Message::*;

        match self {
            KeepAlive => None,
            Choke(data) | Unchoke(data) | Interested(data) | NotInterested(data) | Have(data)
            | Bitfield(data) | Request(data) | Piece(data) | Cancel(data) | Port(data) => {
                Some(data.0)
            }
        }
    }

    /// Get descriptive name of the message variant.
    pub fn get_name(&self) -> &str {
        match self {
            Message::KeepAlive => "Keep alive",
            Message::Choke(_) => "Choke ",
            Message::Unchoke(_) => "Unchoke",
            Message::Interested(_) => "Interested",
            Message::NotInterested(_) => "Not interested",
            Message::Have(_) => "Have",
            Message::Bitfield(_) => "Bitfield",
            Message::Request(_) => "Request",
            Message::Piece(_) => "Piece",
            Message::Cancel(_) => "Cancel",
            Message::Port(_) => "Port",
        }
    }

    /// Returns the payload in a message (or an empty vector, if none).
    pub fn get_payload(&self) -> Vec<u8> {
        use Message::*;

        match self {
            KeepAlive => vec![],
            Choke(data) | Unchoke(data) | Interested(data) | NotInterested(data) | Have(data)
            | Bitfield(data) | Request(data) | Piece(data) | Cancel(data) | Port(data) => {
                data.1.to_owned()
            }
        }
    }

    /// Returns the amount of *bytes* in message.
    pub fn get_length(&self) -> u32 {
        if self.get_id().is_none() {
            0
        } else {
            1 + self.get_payload().len() as u32
        }
    }
}

impl Message {
    /// Construct a "keep-alive" message.
    pub fn new_keep_alive() -> Message {
        Message::KeepAlive
    }

    /// Construct a "choke" message.
    pub fn new_choke() -> Message {
        Message::Choke((0, vec![]))
    }

    /// Construct a "unchoke" message.
    pub fn new_unchoke() -> Message {
        Message::Unchoke((1, vec![]))
    }

    /// Construct a "interested" message.
    pub fn new_interested() -> Message {
        Message::Interested((2, vec![]))
    }

    /// Construct a "not_interested" message.
    pub fn new_not_interested() -> Message {
        Message::NotInterested((3, vec![]))
    }

    /// Construct a "have" message.
    ///
    /// # Arguments
    ///
    /// * `piece_index` - index of piece.
    pub fn new_have(piece_index: u32) -> Message {
        Message::Have((4, piece_index.to_be_bytes().to_vec()))
    }

    /// Construct a "bitfield" message.
    ///
    /// # Arguments
    ///
    /// * `bitfield` - bitfield representing the pieces that have been downloaded.
    pub fn new_bitfield(bitfield: Vec<u8>) -> Message {
        Message::Bitfield((5, bitfield))
    }

    /// Construct a "request" message.
    ///
    /// # Arguments
    ///
    /// * `index` - piece index.
    /// * `begin` - byte offset within the piece.
    /// * `length` - length from byte offset.
    pub fn new_request(index: u32, begin: u32, length: u32) -> Message {
        let mut buf = vec![];
        buf.append(&mut index.to_be_bytes().to_vec());
        buf.append(&mut begin.to_be_bytes().to_vec());
        buf.append(&mut length.to_be_bytes().to_vec());

        Message::Request((6, buf))
    }

    /// Construct a "piece" message.
    ///
    /// # Arguments
    ///
    /// * `index` - piece index.
    /// * `begin` - byte offset within the piece.
    /// * `block` - piece data.
    pub fn new_piece(index: u32, begin: u32, mut block: Vec<u8>) -> Message {
        let mut buf = vec![];
        buf.append(&mut index.to_be_bytes().to_vec());
        buf.append(&mut begin.to_be_bytes().to_vec());
        buf.append(&mut block);

        Message::Piece((7, buf))
    }

    /// Construct a "cancel" message.
    ///
    /// # Arguments
    ///
    /// * `index` - piece index.
    /// * `begin` - byte offset within the piece.
    /// * `length` - length from byte offset.
    pub fn new_cancel(index: u32, begin: u32, length: u32) -> Message {
        let mut buf = vec![];
        buf.append(&mut index.to_be_bytes().to_vec());
        buf.append(&mut begin.to_be_bytes().to_vec());
        buf.append(&mut length.to_be_bytes().to_vec());

        Message::Cancel((8, buf))
    }

    /// Construct a "port" message.
    ///
    /// # Arguments
    ///
    /// * `port` - listen port.
    pub fn new_port(port: u16) -> Message {
        Message::Port((9, port.to_be_bytes().to_vec()))
    }
}
