//! Implementations of serialization traits for various types

use std::net::IpAddr;
use rustc_serialize::{Decoder, Decodable};
use rustc_serialize::{Encoder, Encodable};

use super::announce::{Announce, Event};
use super::{PeerId, InfoHash};

impl Decodable for PeerId {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, D::Error> {
        let hash = try!(decoder.read_str());
        hash.parse().map_err(|_| decoder.error("Malformed peer id"))
    }
}

impl Encodable for PeerId {
    fn encode<S: Encoder>(&self, encoder: &mut S) -> Result<(), S::Error> {
        encoder.emit_str(self.as_ref())
    }
}

impl Decodable for InfoHash {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, D::Error> {
        let hash = try!(decoder.read_str());
        hash.parse().map_err(|_| decoder.error("Malformed info hash"))
    }
}

impl Encodable for InfoHash {
    fn encode<S: Encoder>(&self, encoder: &mut S) -> Result<(), S::Error> {
        encoder.emit_str(self.as_ref())
    }
}

/// A wrapper around `IpAddr` allowing (de)serialization.
struct SerializableIpAddr(pub IpAddr);

impl Decodable for SerializableIpAddr {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, D::Error> {
        let ip = try!(decoder.read_str());
        match ip.parse() {
            Ok(v) => Ok(SerializableIpAddr(v)),
            Err(_) => Err(decoder.error("Malformed id adress")),
        }
    }
}

impl Encodable for SerializableIpAddr {
    fn encode<S: Encoder>(&self, encoder: &mut S) -> Result<(), S::Error> {
        encoder.emit_str(format!("{}", self.0).as_ref())
    }
}

/// Possible values for the `event` field of the announce
const EVENT_NAMES: &'static [&'static str] = &["started", "downloaded"];

impl Decodable for Event {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, D::Error> {
        decoder.read_enum("Event", move |this| {
            this.read_enum_variant(EVENT_NAMES, move |_, index| {
                Ok(match index {
                    0 => Event::Started,
                    1 => Event::Downloaded,
                    _ => unreachable!()
                })
            })
        })
    }
}

impl Decodable for Announce {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, D::Error> {
        decoder.read_struct("Announce", 9, |decoder| {
            Ok(Announce {
                info_hash: try!(decoder.read_struct_field("info_hash", 0, |d| Decodable::decode(d))),
                peer_id: try!(decoder.read_struct_field("peer_id", 1, |d| Decodable::decode(d))),
                ip: try!(decoder.read_struct_field("ip", 2, |decoder| {
                    SerializableIpAddr::decode(decoder).map(|ser_ip| ser_ip.0)
                })),
                port: try!(decoder.read_struct_field("port", 3, |d| Decodable::decode(d))),
                uploaded: try!(decoder.read_struct_field("uploaded", 4, |d| Decodable::decode(d))),
                downloaded: try!(decoder.read_struct_field("downloaded", 5, |d| Decodable::decode(d))),
                left: try!(decoder.read_struct_field("left", 6, |d| Decodable::decode(d))),
                compact: try!(decoder.read_struct_field("compact", 7, |decoder| {
                    Option::<bool>::decode(decoder).map(|opt_b| opt_b.unwrap_or(false))
                })),
                event: try!(decoder.read_struct_field("event", 8, |d| Decodable::decode(d)))
            })
        })
    }
}

#[cfg(test)]
mod tests {
    use rustc_serialize::json;
    use super::super::announce::{Announce, Event};

    #[test]
    fn decode_announce() {
        let json = r#"{
        	"info_hash": "a415ab5cc17c8c093c015ccdb7e552aee7911aa4",
        	"peer_id": "-TR292Z-xa8w76ay04uq",
        	"ip": "127.0.0.1",
        	"port": 9999,
        	"uploaded": 0,
        	"downloaded": 0,
        	"left": 570000,
        	"compact": true,
        	"event": "started"
        }"#;

        let announce = Announce {
            info_hash: "a415ab5cc17c8c093c015ccdb7e552aee7911aa4".parse().unwrap(),
            peer_id: "-TR292Z-xa8w76ay04uq".parse().unwrap(),
            ip: "127.0.0.1".parse().unwrap(),
            port: 9999,
            uploaded: 0,
            downloaded: 0,
            left: 570000,
            compact: true,
            event: Some(Event::Started),
        };

        assert_eq!(json::decode(json), Ok(announce));
    }
}