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


    stream.write(b"Hello world!").unwrap();
    stream.flush().unwrap();

    println!("I wrote 'Hello world!'");

    let mut buf: rustcp::Buffer = vec![];
    stream.read(&mut buf).unwrap();

    if let Ok(res) = rustcp::buf_to_string(&buf){
        println!("I got back: {:?}", res);
    }



    // println!("TCP stream: {:?}", stream);

}

