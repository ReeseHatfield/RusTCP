use std::{
    collections::{hash_map, HashMap},
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    string, vec,
};

use RusTCP::rustcp::{self, buf_to_string, Buffer, RustChatError, SocketAddr};

fn main() {
    // a tcp listener is effecitvely just a server

    let bind_socket_addr = "127.0.0.1:34254"; // must match client socket

    let listener = match TcpListener::bind(bind_socket_addr) {
        Ok(server) => server,
        Err(_) => {
            println!("Could not bind tcp listen to socket {:?}", bind_socket_addr);
            return;
        }
    };

    println!("Sucesfully opened TCP server");

    let mut server: Server = Server {
        connections: HashMap::new(),
        chats: vec![],
    };

    println!("Listening for TCP streams...");
    for possible_streams in listener.incoming() {
        // should pass to handle fn
        // handler to return result to get rid of all of these unwraps with -> ?
        let stream = possible_streams.unwrap(); // TODO fix me

        match handle_incoming(&stream, &mut server) {
            Ok(()) => println!("Sucessfully handled incoming TCP stream"),
            Err(err) => println!("Could not handle TCP stream, {:?}", err),
        };
    }
}

fn handle_incoming(mut stream: &TcpStream, server: &mut Server) -> Result<(), RustChatError> {
    let mut buf: rustcp::Buffer = vec![0; 1024];

    let cur_addr: rustcp::SocketAddr = stream.peer_addr().unwrap().into();

    if !server.connections.contains_key(&cur_addr) {
        server.register_connection(&cur_addr, stream);
    }

    // add chat to vec
    // then broadcast

    let num_bytes_read = stream.read(&mut buf);

    // should just maintain a copy of the chats message
    let chat = Chat {
        message: buf.clone(),
        source: cur_addr,
    };

    stream
        .write_all(&buf)
        .map_err(|_| RustChatError::TcpStreamError("Could not write to TCP stream".to_string()))?;
    stream.flush().unwrap();
    // buf.clear();

    println!("All chat so far:");
    server.print_all_chats();

    std::thread::sleep(std::time::Duration::from_secs(3));
    server.notify_all(&chat);

    server.chats.push(chat);
    Ok(())
}

struct Server {
    // wanna be able to lookup stream from socket later
    connections: HashMap<rustcp::SocketAddr, TcpStream>,
    chats: Vec<Chat>,
}

impl Server {
    pub fn register_connection(&mut self, connection: &SocketAddr, stream: &TcpStream) {
        // TODO idk what the fail condition is here, but should be handled eventually
        self.connections
            .insert(connection.clone(), stream.try_clone().unwrap());
    }

    pub fn print_all_chats(&self) {
        self.chats.iter().for_each(|chat| {
            println!("Message: {:?}", buf_to_string(&chat.message).unwrap()); // DEBUG ONLY unwrap is fine here
            println!("Source: {:?}", chat.source);
            println!();
        });
    }
    pub fn notify_all(&self, chat: &Chat) {
        // filter out everyone else from the map
        let everyone_else: Vec<&TcpStream> = self
            .connections
            .iter()
            .filter(|(con, _)| **con != chat.source)
            .map(|(_, stream)| stream)
            .collect();

        for mut stream in everyone_else {
            stream
                .write_all(&chat.message)
                .map_err(|_| RustChatError::TcpStreamError("Could not send message".to_string()))
                .unwrap();
        }
    }
}

struct Chat {
    message: Buffer,
    source: rustcp::SocketAddr,
}

impl std::fmt::Display for Chat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match buf_to_string(&self.message) {
            Ok(s) => write!(f, "{}", s),
            Err(_) => write!(f, "Error: could not render messsage"),
        }
    }
}

// a connection TO the TCP server
struct Connection {
    client: rustcp::SocketAddr,
}
