use crate::Message;

impl Message {
    /// Converts a `Message` into a byte vector.
    pub fn into_bytes(self) -> Vec<u8> {
        let mut out = vec![];

        out.append(&mut self.get_length().to_be_bytes().to_vec());

        if let Some(id) = self.get_id() {
            out.push(id);
        }

        out.append(&mut self.get_payload());

        out
    }
}
