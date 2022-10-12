#[macro_use]
extern crate arrayref;

mod bcode;
mod cli;
mod client;
mod tests;
mod torrent;

use cli::Cli;
use client::Client;
use torrent::Torrent;

use anyhow::{anyhow, Result};
use clap::Parser;
use std::convert::TryFrom;

fn main() -> Result<()> {
    let args = Cli::parse();

    if let Ok(mut bytes) = std::fs::read(args.path) {
        let torrent = Torrent::try_from(&mut bytes)?;
        let client = Client::new(torrent)?;

        client.send_tracker_request()?;

        //println!("{:?}", client);
    } else {
        return Err(anyhow!("Failed reading torrent file"));
    }

    Ok(())
}
