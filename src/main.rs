mod tests;
mod cli;
mod decode;

use cli::Cli;
use decode::decode;

use anyhow::{anyhow, Result};
use clap::Parser;

fn main() -> Result<()> {
    let args = Cli::parse();

    if let Ok(bytes) = std::fs::read("torrents/test.torrent") {
        let decoded = decode(bytes).unwrap();
        println!("{:?}", decoded);
    } else {
        return Err(anyhow!("Failed reading torrent file"));
    }

    Ok(())
}
