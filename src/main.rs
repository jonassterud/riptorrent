mod cli;

use anyhow::{anyhow, Result};
use cli::*;
use torrent::Torrent;

#[async_std::main]
async fn main() -> Result<()> {
    let args = cli::Args::parse();

    if let Ok(bytes) = std::fs::read(args.path) {
        let torrent = Torrent::from_bytes(bytes).await?;
        println!("{:?}", torrent);

        Ok(())
    } else {
        Err(anyhow!("Failed reading torrent file"))
    }
}
