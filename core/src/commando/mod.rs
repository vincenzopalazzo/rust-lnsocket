/// commando client implementation
///
/// author: https://github.com/vincenzopalazzo
use super::lnsocket::client::{ClientMessage, LNSocketClient};
use serde::{Deserialize, Serialize};
use std::io::Error;

/// General request
#[derive(Debug, Serialize, Deserialize)]
pub struct CommandoRequest {
    id: u64,
    rune: Option<String>,
    method: String,
    params: Vec<String>,
}

impl CommandoRequest {
    pub fn new(rune: Option<String>, method: String, params: Vec<String>) -> Self {
        CommandoRequest {
            id: 1,
            rune: rune,
            method: method,
            params: params,
        }
    }
}

impl ClientMessage for CommandoRequest {
    fn encode(&self) -> Result<String, Error> {
        match serde_json::to_string(self) {
            Ok(res) => Ok(res),
            Err(err) => panic!("{}", err),
        }
    }

    fn decode(&self, buffer: &str) -> Result<Box<dyn ClientMessage>, Error> {
        let result: CommandoResponse = match serde_json::from_str(buffer) {
            Ok(res) => res,
            Err(err) => panic!("{}", err),
        };
        Ok(Box::new(result))
    }
}

// TODO: implementing it with the response information
#[derive(Debug, Serialize, Deserialize)]
pub struct CommandoResponse;

impl ClientMessage for CommandoResponse {
    fn encode(&self) -> Result<String, Error> {
        todo!()
    }

    fn decode(&self, _buffer: &str) -> Result<Box<(dyn ClientMessage + 'static)>, Error> {
        todo!()
    }
}

/// Implementation of commando client rules
pub struct CommandoClient;

impl LNSocketClient for CommandoClient {
    // TODO: implementing the following method in pure Rust
    // https://github.com/jb55/lnsocket/blob/40911ed4a715f13adb18f13949839a58f64dc18e/commando.c#L8
    fn build_req(&self, request: Box<dyn ClientMessage>) -> Result<String, Error> {
        request.encode()
    }

    // TODO: implementing it
    fn decode_resp(&self, buffer: &str) -> Result<Box<dyn ClientMessage>, Error> {
        Ok(Box::new(CommandoResponse {}))
    }
}
