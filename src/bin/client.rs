use std::{io::{Read, Write}, net::TcpStream, vec};

use RusTCP::rustcp;

fn main() {
    let socket = "127.0.0.1:34254";

    // stream -> connection from client and server
    // stream will get closed whenever drop(stream) is called


    let mut stream = match TcpStream::connect(socket) {
        Ok(stream) => stream,
        Err(_) => {
            println!("Could not open TCP stream at socket {:?}", socket);
            return;
        }
    };


    let message = "Here is some wierd other data";

    stream.write(message.as_bytes()).unwrap();
    stream.flush().unwrap();

    println!("I wrote {:?}", message);

    let mut buf: rustcp::Buffer = vec![0; 1024];
    stream.read(&mut buf).unwrap();

    // println!("BUF: {:?}", buf);

    if let Ok(res) = rustcp::buf_to_string(&buf){
        println!("I got back: {:?}", res);
    }



    // println!("TCP stream: {:?}", stream);

}

