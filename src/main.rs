mod cli;

use anyhow::{anyhow, Result};
use cli::*;
use torrent::Torrent;

#[async_std::main]
async fn main() -> Result<()> {
    let args = cli::Args::parse();

    if let Ok(bytes) = std::fs::read(args.path) {
        let torrent = Torrent::from_bytes(bytes).await?;
        let tracker_request =
            tracker::Request::from_torrent(&torrent, b"j9jkjkj9jshdhghfj398".to_vec()).await;
        let tracker_resp = tracker_request.send_request().await?;

        println!("{:?}", tracker_resp);

        Ok(())
    } else {
        Err(anyhow!("Failed reading torrent file"))
    }
}
