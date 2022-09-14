use clap::Parser;

/// CLI struct for Clap
#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    /// Path to torrent file
    pub path: String,
}
