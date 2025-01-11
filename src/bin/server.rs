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

    let server = std::sync::Arc::new(std::sync::Mutex::new(Server {
        connections: HashMap::new(),
        chats: vec![],
    }));

    println!("Listening for TCP streams...");
    for possible_streams in listener.incoming() {
        // should pass to handle fn
        // handler to return result to get rid of all of these unwraps with -> ?
        let stream = possible_streams.unwrap(); // TODO fix me

        let server_clone = std::sync::Arc::clone(&server);

        // handle the client in some other thread
        std::thread::spawn(move || handle_incoming(stream, server_clone));
    }
}

fn handle_incoming(
    mut stream: TcpStream,
    server: std::sync::Arc<std::sync::Mutex<Server>>,
) -> Result<(), RustChatError> {
    let cur_addr: rustcp::SocketAddr = stream
        .peer_addr()
        .map_err(|e| {
            RustChatError::TcpStreamError("Could not read peer socket address".to_string())
        })?
        .into();
    let mut buf: rustcp::Buffer = vec![0; 1024];

    {
        // need the server to drop scope do it doesnt keep its lock
        let mut server = server.lock().map_err(|_| {
            RustChatError::TcpThreadLockError("Could not aquire server lock".to_string())
        })?;
        if !server.connections.contains_key(&cur_addr) {
            server.register_connection(&cur_addr, &stream);
        }
    }

    loop {
        match stream.read(&mut buf) {
            // this is new to me, but thats really cool that you can do that
            Ok(0) => {
                // vast majoritt of
                println!("Client disconnected");
                break;
            }
            Ok(n) => {
                println!("Got some bytes");
                n
            }
            Err(err) => {
                println!("Error reading from stream: {:?}", err);
                break;
            }
        };

        let chat = Chat {
            message: buf.clone(),
            source: cur_addr.clone(),
        };

        {
            let mut server = server.lock().map_err(|_| {
                RustChatError::TcpThreadLockError("Could not aquire server lock".to_string())
            })?;
            server.chats.push(chat.clone());
            server.notify_all(&chat);
        }

        println!(
            "Received message: {:?}",
            match buf_to_string(&chat.message){
                Ok(msg) => println!("{:?}", msg),
                Err(e) => eprintln!("{:?}", e),
            }
        );
    }

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

    // DEBUG ONLY unwrap is fine here
    pub fn print_all_chats(&self) {
        self.chats.iter().for_each(|chat| {
            println!("Message: {:?}", buf_to_string(&chat.message).unwrap()); 
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

#[derive(Clone)]
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
