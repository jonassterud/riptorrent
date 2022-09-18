mod cli;
mod decode;
mod tests;
mod torrent;

use std::convert::TryFrom;

use cli::Cli;
use decode::{decode, Value};
use torrent::Torrent;

use anyhow::{anyhow, Result};
use clap::Parser;

fn main() -> Result<()> {
    let args = Cli::parse();

    if let Ok(mut bytes) = std::fs::read(args.path) {
        let decoded = decode(&mut bytes, &mut 0).unwrap();
        let torrent = Torrent::try_from(decoded.get(0).unwrap())?;

        println!("{:?}", torrent);
    } else {
        return Err(anyhow!("Failed reading torrent file"));
    }

    Ok(())
}
