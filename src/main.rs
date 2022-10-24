mod cli;

use anyhow::{anyhow, Result};
use cli::*;
use torrent::Torrent;

#[async_std::main]
async fn main() -> Result<()> {
    let args = cli::Args::parse();

    if let Ok(bytes) = std::fs::read(args.path) {
        let torrent = Torrent::from_bytes(bytes).await?;

        let peer_id = b"j9jkjkj9jshdhghfj398".to_vec();
        let tracker_request = tracker::Request::from_torrent(&torrent, peer_id).await;
        let tracker_resp = tracker_request.send_request().await?;

        for peer in tracker_resp.peers {
            async_std::task::spawn(async move { todo!() });
        }

        // wait
        std::io::stdin().read_line(&mut String::new()).unwrap();

        Ok(())
    } else {
        Err(anyhow!("Failed reading torrent file"))
    }
}
