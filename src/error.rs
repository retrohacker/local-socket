use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum SocketError {
    ReadFailure(std::io::Error),
    WriteFailure(std::io::Error),
    AcceptFailure(std::io::Error),
    ConnectFailure(std::io::Error),
    SocketClosed,
    DataCorrupt,
    BufferOverflow,
}

impl Display for SocketError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}

impl std::error::Error for SocketError {}
