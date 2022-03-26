/// Public API of LNSocket
///
/// author: https://github.com/vincenzopalazzo
use crate::lnsocket::bindings::*;
use crate::lnsocket::client::{LNSocketClient, Request};
use std::ffi::CString;
use std::io::Error;

pub struct LNSocket {
    ln_socket: lnsocket,
    client: Box<dyn LNSocketClient>,
}

impl LNSocket {
    /// Create a new instance on LNSocket
    pub fn new(client: Box<dyn LNSocketClient>) -> Self {
        let socket = unsafe { lnsocket_create() };
        return LNSocket {
            ln_socket: unsafe { *socket },
            client,
        };
    }

    /// Connect to the provided host
    pub fn connect(&mut self, node_id: &str, host: &str) -> bool {
        if !self.internal_connect(node_id, host) {
            return false;
        }
        self.internal_initialization()
    }

    pub fn send_msg(&mut self, request: &Request) -> Result<(), Error> {
        let msg = self.client.build_req(request);
        let mut buff = String::new();
        match msg {
            Ok(buffer) => buff = buffer,
            Err(err) => return Err(err),
        };
        let c_msg = CString::new(buff.as_bytes()).unwrap();
        let result = unsafe {
            let cmsg = c_msg.as_bytes().as_ptr();
            lnsocket_write(&mut self.ln_socket, cmsg, buff.len() as u16)
        };
        // TODO check this and return an error if necessary
        assert_eq!(result, 0);
        Ok(())
    }

    /// Perform the init between the host node and the own node
    /// implementation init as specified in the BOL1
    /// https://github.com/lightning/bolts/blob/master/01-messaging.md#the-init-message
    fn internal_initialization(&mut self) -> bool {
        let result = unsafe { lnsocket_perform_init(&mut self.ln_socket) };
        result == 0
    }

    /// perform the internal connect
    fn internal_connect(&mut self, node_id: &str, host: &str) -> bool {
        let c_node_id = CString::new(node_id).unwrap();
        let c_host = CString::new(host).unwrap();
        let result = unsafe {
            let nodeid = c_node_id.as_ptr();
            let node_host = c_host.as_ptr();
            lnsocket_connect(&mut self.ln_socket, nodeid, node_host)
        };
        result == 0
    }
}
