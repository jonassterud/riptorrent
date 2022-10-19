//! # Torrent
//! 
//! `torrent` is a library for decoding a `.torrent` file and 
//! transforming it into a data struct.

mod torrent_struct;

pub use torrent_struct::Torrent;