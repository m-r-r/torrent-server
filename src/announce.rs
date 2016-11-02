//! Type definition for bittorrent announces

use std::net::IpAddr;
use super::{PeerId, InfoHash};

#[derive(Debug, Eq, PartialEq)]
/// Value of the `event` field of an announce.
pub enum Event {
    /// The peer has started to download data.
    Started,
    /// The peer has already completed the download.
    Downloaded,
}

#[derive(Debug, Eq, PartialEq)]
/// A bittorrent announce.
pub struct Announce {
    /// The torrent's information hash
    pub info_hash: InfoHash,
    /// A random identifier supplied by the peer
    pub peer_id: PeerId,
    /// The IP adress of the peer
    pub ip: IpAddr,
    /// The port used by the peer
    pub port: usize,
    /// The volume of data uploaded by the peer, in bytes
    pub uploaded: u64,
    /// The volume of data downloaded by the peer, in bytes
    pub downloaded: u64,
    /// The volume of data not yet downloaded by the peer, in bytes
    pub left: u64,
    /// Whether the compact format should be used for the answer
    pub compact: bool,
    /// An optional event
    pub event: Option<Event>,
}