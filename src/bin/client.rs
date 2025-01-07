use std::net::TcpStream;

fn main() {
    let socket = "127.0.0.1:34254";

    // stream -> connection from client and server
    let mut stream = match TcpStream::connect(socket) {
        Ok(stream) => stream,
        Err(_) => {
            println!("Could not open TCP stream at socket {:?}", socket);
            return;
        }
    };

    println!("TCP stream: {:?}", stream);

}
