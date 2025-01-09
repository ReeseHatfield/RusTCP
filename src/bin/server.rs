use std::{io::{Read, Write}, net::TcpListener, string};

use RusTCP::rustcp::{self, buf_to_string, Buffer};

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


struct Server{
    connections: Vec<Connection>,
    chats: Vec<Chat>
}

struct Chat(Buffer);


impl std::fmt::Display for Chat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        match buf_to_string(&self.0){
            Ok(s) => write!(f, "{}", s),
            Err(_) => write!(f, "Error: could not render messsage")
            
        }
        
    }
}


// a connection TO the TCP server
struct Connection {
    client: Socket
}


struct Socket {
    address: Address,
    port: Port,
}


// newtype pattern
struct Address(String);
// can like impl parse on these and do that cool pattern thingy
struct Port(u8);



