use std::{io::{Read, Write}, net::TcpListener};

use RusTCP::rustcp;

fn main(){
    // a tcp listener is effecitvely just a server


    let socket = "127.0.0.1:34254"; // must match client socket

    let server = match TcpListener::bind(socket){
        Ok(server) => {
            server
        }
        Err(_) => {
            println!("Could not bind tcp listen to socket {:?}", socket);
            return
        }
    };

    println!("Sucesfully opened TCP server");


    let mut buf: rustcp::Buffer = vec![0; 1024];

    println!("Listening for TCP streams...");
    for possible_streams in server.incoming(){
        // should pass to handle fn

        let mut stream = possible_streams.unwrap();
        let num_bytes_read = stream.read(&mut buf);

        println!("Buffer: {:?}", buf);
        println!("num_bytes_read: {:?}", num_bytes_read);

        stream.write_all(&buf).unwrap();
        stream.flush().unwrap(); 
        // buf.clear();

    }


}