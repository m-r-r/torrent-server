//! A simple torrent tracker

extern crate rustc_serialize;
extern crate bencode;

mod announce;
mod response;
mod serialize;

use std::net::IpAddr;
use std::str::FromStr;
use std::ascii::AsciiExt;

#[derive(Debug, Eq, PartialEq)]
/// The information hash of a torrent.
pub struct InfoHash(String);

#[derive(Debug, Eq, PartialEq)]
/// A random string used to identify a peer.
pub struct PeerId(String);

#[derive(Debug, Eq, PartialEq)]
/// Indicates a malformed peer id.
pub struct PeerIdDecodeError;

#[derive(Debug, Eq, PartialEq)]
/// Indicates a malformed information hash.
pub struct InfoHashDecodeError;

impl FromStr for PeerId {
    type Err = PeerIdDecodeError;

    fn from_str(hash: &str) -> Result<Self, Self::Err> {
        if hash.len() <= 20 {
            Ok(PeerId(hash.to_string()))
        } else {
            Err(PeerIdDecodeError)
        }
    }
}

impl AsRef<str> for PeerId {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl Into<String> for PeerId {
    fn into(self) -> String {
        self.0
    }
}

impl FromStr for InfoHash {
    type Err = InfoHashDecodeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let hash = s.to_ascii_lowercase();
        if hash.len() == 40 {
            Ok(InfoHash(hash))
        } else {
            Err(InfoHashDecodeError)
        }
    }
}

impl AsRef<str> for InfoHash {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl Into<String> for InfoHash {
    fn into(self) -> String {
        self.0
    }
}

/// A torrent peer.
pub struct Peer {
    /// The random string used to identify the peer.
    pub peer_id: PeerId,
    /// The peer's ip adress.
    pub ip: IpAddr,
    /// The peer's port.
    pub port: usize,
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use super::*;

    #[test]
    fn parse_peer_id() {
        assert_eq!(PeerId::from_str("-TR292Z-VxeLvpTD5X90").map(|p| p.into()), Ok("-TR292Z-VxeLvpTD5X90".to_string()));
        assert_eq!(PeerId::from_str("-TR292Z-VxeLvpTD5X90x"), Err(PeerIdDecodeError));
    }

    #[test]
    fn parse_info_hash() {
        assert_eq!(InfoHash::from_str("a94A8fe5CCb19ba61c4c0873d391E987982fbbd3").map(|p| p.into()), Ok("a94a8fe5ccb19ba61c4c0873d391e987982fbbd3".to_string()));
        assert_eq!(InfoHash::from_str("a94a8fe5ccb19ba61c4c0873d391e987982fbbd3lol"), Err(InfoHashDecodeError));
    }
}
