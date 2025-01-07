use std::{io::{Read, Write}, net::TcpListener};

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


    let mut buf = vec![];

    println!("Listening for TCP streams...");
    for stream in server.incoming(){
        // should pass to handle fn

        let num_bytes_read = stream.unwrap().read_to_end(&mut buf);

        println!("Buffer: {:?}", buf);
        println!("num_bytes_read: {:?}", num_bytes_read);


    }


}