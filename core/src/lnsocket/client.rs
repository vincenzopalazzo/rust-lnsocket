/// Client abstract about the possibility to send
/// LN message and communicate with the ln node.
///
/// a possible client rules is designed by a plugin
/// like commando: https://github.com/lightningd/plugins/tree/master/commando
///
/// author: https://github.com/vincenzopalazzo
use std::io::Error;

/// Public interface to implement any type of
/// client over the lnsocket client.
pub trait LNSocketClient {
    /// build the buffer to send throughout the socket
    fn build_req(&self, request: Box<dyn ClientMessage>) -> Result<String, Error>;
    /// decode the buffer received from the socket;
    fn decode_resp(&self, buffer: &str) -> Result<Box<dyn ClientMessage>, Error>;
}

pub trait ClientMessage {
    /// encode the request into a string
    fn encode(&self) -> Result<String, Error>;
    /// decode a request from a string
    fn decode(&self, buffer: &str) -> Result<Box<dyn ClientMessage>, Error>;
}
