use super::*;
use std::thread;

impl Client {
    pub fn start(self) {
        for mut peer in self.last_response.unwrap().peers.unwrap() {
            
            thread::spawn(move || {
                // Send handshake
                peer.send(Client::get_handshake(
                    peer.info_hash.clone().unwrap(),
                    peer.peer_id.clone().unwrap_or_default(),
                ).unwrap()).unwrap();

                // Receive handshake
                let mut buf: [u8; 68] = [0; 68];
                peer.read_exact(&mut buf).unwrap();
                println!("{:?}", peer);
                println!("handshake: {:?}", buf);
            });

            /* 
            thread::spawn(async move {
        
                // Send handshake
                peer.send(Client::get_handshake(
                    peer.info_hash.clone().unwrap(),
                    peer.peer_id.clone().unwrap_or_default(),
                ).unwrap()).await.unwrap();

                // Receive handshake
                let mut buf = vec![];
                peer.read_to_end(&mut buf).await.unwrap();
                println!("{:?}", buf);

               
            });
            */
        }
    }
}
