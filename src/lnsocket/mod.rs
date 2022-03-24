/// lnsocket crate
/// core implementation of the FFI binding for C library
///
/// author: https://github.com/vincenzopalazzo

#[link(name = "lnsocket")]
extern "C" {}

struct LNSocket {}

impl LNSocket {
    /// Create a new instance on LNSocket
    pub fn new() -> Self {
        return LNSocket {};
    }
}
