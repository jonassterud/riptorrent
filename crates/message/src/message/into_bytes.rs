use crate::Message;

impl Message {
    /// Converts a `Message` into a byte vector.
    pub fn into_bytes(self) -> Vec<u8> {
        let mut out = vec![];

        out.append(&mut self.length.to_be_bytes().to_vec());

        if self.message_id.is_some() {
            out.push(self.message_id.unwrap());
        }

        if self.payload.is_some() {
            out.append(&mut self.payload.unwrap());
        }

        out
    }
}
