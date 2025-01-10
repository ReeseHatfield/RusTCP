use std::{
    io::{stdin, Read, Write},
    net::TcpStream,
    vec,
};

use RusTCP::rustcp::{self, RustChatError, SocketAddr, Port, IP_Address};

fn main() -> Result<(), rustcp::RustChatError> {
    // let socket = "127.0.0.1:34254";

    let socket_addr = read_socket_addr()?;
    // stream -> connection from client and server
    // stream will get closed whenever drop(stream) is called

    let mut stream = open_stream(socket_addr)?;

    let message = "Here is some wierd other data";

    stream.write(message.as_bytes()).unwrap();
    stream.flush().unwrap();

    println!("I wrote {:?}", message);

    let mut buf: rustcp::Buffer = vec![0; 1024];
    stream.read(&mut buf).unwrap();

    // println!("BUF: {:?}", buf);

    if let Ok(res) = rustcp::buf_to_string(&buf) {
        println!("I got back: {:?}", res);
    }

    println!("TCP stream: {:?}", stream);

    Ok(())
}

fn read_socket_addr() -> Result<rustcp::SocketAddr, rustcp::RustChatError> {
    let mut input_string = String::new();
    println!("Please enter the socket in the form IP:PORT (enter for default)");

    // wanna match on the input here

    if stdin().read_line(&mut input_string).is_err() {
        return Err(rustcp::RustChatError::SocketParseError(
            "Could not read socket from user".to_owned(),
        ));
    }

    match input_string.trim(){
        "" =>  Ok(SocketAddr{ ip_addr: IP_Address("127.0.0.1".to_string()), port: Port(34254) }),
        _ => input_string.trim().parse::<rustcp::SocketAddr>()
    }
}

fn open_stream(socket: SocketAddr) -> Result<TcpStream, RustChatError> {
    let bind_addr: String = socket.ip_addr.0 + ":" + socket.port.0.to_string().as_str();

    let stream = TcpStream::connect(bind_addr)
        .map_err(|_| RustChatError::TcpStreamError("Could not open TCP stream".to_string()));

    return stream;
}
