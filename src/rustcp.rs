// shared between both client and server

use core::str;
use std::str::FromStr;

use std::net::{SocketAddr as StdSocketAddr, IpAddr as StdIpAddr};

pub type Buffer = Vec<u8>;

#[derive(Debug,PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct SocketAddr {
    pub ip_addr: IP_Address,
    pub port: Port,
}

impl FromStr for SocketAddr {
    type Err = RustChatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(":");

        // ok_or_(else) is just Option -> Result
        let addr_str = parts
            .next()
            .ok_or_else(|| RustChatError::SocketParseError("Missing address".to_string()))?;
        let port_str = parts
            .next()
            .ok_or_else(|| RustChatError::SocketParseError("Missing port".to_string()))?;

        let addr: IP_Address = addr_str.parse()?;
        let port: Port = port_str.parse()?;

        Ok(SocketAddr {
            ip_addr: addr,
            port: port,
        })
    }
}
// newtype pattern
// can like impl parse on these and do that cool pa("Could not parse addresss".to_owned())ttern thingy
// parse, dont validate

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct IP_Address(pub String);

impl FromStr for IP_Address {
    type Err = RustChatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // TODO add actual ip parsing here
        Ok(IP_Address(s.to_string()))
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Port(pub u16);

impl FromStr for Port {
    type Err = RustChatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {

        println!("When parsing port, i see: {:?}", s);
        let port_num = s
            .parse::<u16>()
            .map_err(|_| RustChatError::SocketParseError(format!("Could not parse port")))?;

        return Ok(Port(port_num));
    }
}

pub fn buf_to_string(buf: &Buffer) -> Result<String, RustChatError> {
    let trimed_content: Vec<u8> = buf.iter().filter(|c| **c != 0).map(|&x| x).collect();

    let content = str::from_utf8(&trimed_content).map_err(|_| {
        RustChatError::BufferConversionError(format!("Could not convert buffer into string"))
    })?;

    return Ok(content.trim().to_string());
}

#[derive(Debug)]
pub enum RustChatError {
    BufferConversionError(String),
    SocketParseError(String),
    TcpStreamError(String)
}

impl std::error::Error for RustChatError {}

impl std::fmt::Display for RustChatError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TcpStreamError(msg) => write!(f, "Error in TCP stream: {}", msg),
            Self::SocketParseError(msg) => write!(f, "Socket parsing error: {}", msg),
            Self::BufferConversionError(msg) => write!(f, "Buffer conversion error: {}", msg),
        }
    }
}


// I need the server to track connections, so It just makes stuff easier If have my own type serverside, with ordering
// I need to convert between them
impl From<StdSocketAddr> for SocketAddr {
    fn from(std_socket_addr: StdSocketAddr) -> Self {
        let ip_addr = match std_socket_addr.ip() {
            StdIpAddr::V4(ipv4) => IP_Address(ipv4.to_string()),
            StdIpAddr::V6(ipv6) => IP_Address(ipv6.to_string()),
        };
        let port = Port(std_socket_addr.port());

        SocketAddr {
            ip_addr,
            port,
        }
    }
}