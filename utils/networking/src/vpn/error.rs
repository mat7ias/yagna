use std::net::{AddrParseError, IpAddr};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Invalid IP address: {0}")]
    IpAddr(#[from] AddrParseError),
    #[error("IP address not allowed: {0}")]
    IpAddrNotAllowed(IpAddr),
    #[error("IP address taken: {0}")]
    IpAddrTaken(IpAddr),
    #[error("Invalid network IP address: {0}")]
    NetAddr(String),
    #[error("Network IP address taken: {0}")]
    NetAddrTaken(IpAddr),
    #[error("Network not found for IP address: {0}")]
    NetAddrMismatch(IpAddr),
    #[error("Network is empty")]
    NetEmpty,
    #[error("Network not found: {0}")]
    NetNotFound(String),
    #[error("Invalid network CIDR: {0}/{1}")]
    NetCidr(IpAddr, u8),
    #[error("Network ID taken: {0}")]
    NetIdTaken(String),
    #[error("Packet malformed: {0}")]
    PacketMalformed(String),
    #[error("Protocol not supported: {0}")]
    ProtocolNotSupported(String),
    #[error("{0}")]
    Other(String),
}
