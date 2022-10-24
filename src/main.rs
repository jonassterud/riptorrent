mod cli;

use anyhow::{anyhow, Result};
use cli::*;
use torrent::Torrent;

#[async_std::main]
async fn main() -> Result<()> {
    let args = cli::Args::parse();

    if let Ok(bytes) = std::fs::read(args.path) {
        let torrent = Torrent::from_bytes(bytes).await?;

        let info_hash = torrent.info_hash.to_owned();
        let peer_id = b"j9jkjkj9jshdhghfj398".to_vec();
        
        let tracker_request = tracker::Request::from_torrent(&torrent, &peer_id).await;
        let tracker_resp = tracker_request.send_request().await?;

        println!("Peer amount: {}", tracker_resp.peers.len());

        for peer in tracker_resp.peers {
            let info_hash = info_hash.clone();
            let peer_id = peer_id.clone();

            async_std::task::spawn(async move {
                let mut stream = protocol::open_stream(
                    peer.ip.ok_or_else(|| anyhow!("Missing ip"))?,
                    peer.port.ok_or_else(|| anyhow!("Missing port"))?,
                )
                .await?;

                println!("Opened stream");

                protocol::send_handshake(&mut stream, &info_hash, &peer_id).await?;
                let peer_handshake = protocol::read_handshake(&mut stream).await?;
                println!("Received handshake!");

                println!("Handshake: {:?}", peer_handshake);

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
