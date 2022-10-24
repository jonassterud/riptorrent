pub use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// Path to torrent file
    pub path: String,
}
