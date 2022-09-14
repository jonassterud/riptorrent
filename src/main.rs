mod cli;
mod decode;
mod tests;

use cli::Cli;
use decode::decode;

use anyhow::{anyhow, Result};
use clap::Parser;

fn main() -> Result<()> {
    let args = Cli::parse();

    if let Ok(mut bytes) = std::fs::read(args.path) {
        let decoded = decode(&mut bytes, &mut 0).unwrap();
        // ...
    } else {
        return Err(anyhow!("Failed reading torrent file"));
    }

    Ok(())
}
