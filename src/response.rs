use std::error::Error;
use super::Peer;

pub enum Response {
    /// The request failed
    Error {
        /// An optional error message
        failure_reason: Option<String>,
    },
    /// A list of connected peers
    PeerList {
        /// Refresh interval of the list
        interval: u32,
        /// Available peers
        peers: Vec<Peer>,
    },
}

impl<E> From<E> for Response
    where E: Error
{
    fn from(error: E) -> Self {
        Response::Error {
            failure_reason: Some(format!("{}", error)),
        }
    }
}