/// Client abstract about the possibility to send
/// LN message and communicate with the ln node.
///
/// a possible client rules is designed by a plugin
/// like commando: https://github.com/lightningd/plugins/tree/master/commando
///
/// author: https://github.com/vincenzopalazzo
use super::core::LNSocket;
use std::io::Error;

/// Public interface to implement any type of
/// client over the lnsocket client.
pub trait LNSocketClient {
    /// build the buffer to send throughout the socket
    fn build_req(&self, request: &Request) -> Result<String, Error>;
    /// decode the buffer received from the socket;
    fn decode_resp(&self, buffer: &str) -> Result<Response, Error>;
}

/// General request
pub struct Request {
    id: u64,
    rune: Option<String>,
    method: String,
    params: String,
}

// TODO: implementing it with the response information
pub struct Response {}

/// Implementation of commando client rules
pub struct CommandoClient;

impl LNSocketClient for CommandoClient {
    // TODO: implementing the following method in pure Rust
    // https://github.com/jb55/lnsocket/blob/40911ed4a715f13adb18f13949839a58f64dc18e/commando.c#L8
    fn build_req(&self, request: &Request) -> Result<String, Error> {
        Ok("".to_string())
    }

    // TODO: implementing it
    fn decode_resp(&self, buffer: &str) -> Result<Response, Error> {
        Ok(Response {})
    }
}
