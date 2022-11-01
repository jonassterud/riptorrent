mod cli;

use anyhow::{anyhow, Result};
use async_std::sync::{Arc, Mutex};
use builder::Builder;
use cli::*;
use torrent::Torrent;

// TODO list:
//
// * IPv6 not working? - find workaround for networks that don't have 6rd or similar.
// * Generate a peer id.
// * Create piece download strategy.

#[async_std::main]
async fn main() -> Result<()> {
    let args = cli::Args::parse();

    if let Ok(bytes) = std::fs::read(args.path) {
        // Open torrent and get information from tracker.
        let peer_id = b"-qBhj010488887635243".to_vec();
        let torrent = Torrent::from_bytes(bytes).await?;
        let tracker = tracker::Request::from_torrent(&torrent, &peer_id).await;
        let tracker_resp = tracker.send_request().await?;

        // Create builder
        let builder = Arc::new(Mutex::new(Builder::new(
            torrent.get_piece_amount(),
            torrent.get_piece_length() as usize,
            u32::pow(2, 14) as usize,
        )));

        // Loop trough peers.
        for (i, mut peer) in tracker_resp.peers.into_iter().enumerate() {
            if i > 25 {
                break;
            }

            // Clone values.
            let mut info_hash = torrent.info_hash.clone();
            let mut id = peer_id.clone();
            let piece_amount = torrent.get_piece_amount();
            let builder = builder.clone();

            // Spawn an async task.
            async_std::task::spawn(async move {
                peer.setup(&mut info_hash, &mut id, piece_amount).await?;
                println!("Ready with {:?}", peer.ip);
                peer.start(builder).await?;

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
