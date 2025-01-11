use std::{
    io::{self, stdin, Read, Write}, net::TcpStream, sync::{Arc, Mutex}, thread, vec
};

use RusTCP::rustcp::{self, IP_Address, Port, RustChatError, SocketAddr};

fn main() -> Result<(), rustcp::RustChatError> {


    let socket_addr = read_socket_addr()?;
    // stream -> connection from client and server
    // stream will get closed whenever drop(stream) is called
    // socket -> 4 tuple (SRC_IP, SRC_PORT, DST_IP, DST_PORT)
    // OR socket -> (SRC_SOCKET_ADDR, DSR_SOCKET_ADDR)
    // where socket_addr -> (IP:PORT)

    let shared_stream = open_stream(socket_addr)?;
    shared_stream.set_nonblocking(true).map_err(|_| RustChatError::TcpStreamError("Could not set stream non-blocking".to_string()))?;
    let shared_stream = Arc::new(Mutex::new(shared_stream)); // will coerce type


    let is_running = Arc::new(Mutex::new(true));
    // let message = "new input";

    let receiving_stream = Arc::clone(&shared_stream);
    let receiver_running = Arc::clone(&is_running);
    let rec_thread = thread::spawn(move || {

        let mut buf: rustcp::Buffer = vec![0; 1024];

        while *receiver_running.lock().unwrap() {
            let mut stream = receiving_stream.lock().unwrap();

            match stream.read(&mut buf) {
                Ok(actual_data) if actual_data > 0 => {
                    let message = rustcp::buf_to_string(&buf).unwrap();

                    println!("Server: {}", message);
                },

                Ok(_) => {},// continue, found nothing back
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                    // read() will sometimes do this instead, but I dont care
                }
                Err(_) => {
                    println!("Server error oopsies");
                    break;
                }
            }
        }
    });

    let sending_stream = Arc::clone(&shared_stream);
    let sender_running = Arc::clone(&is_running);

    let send_thread = thread::spawn(move || {
        let stdin = io::stdin();
        let mut stdout = io::stdout();
        let mut input_str = String::new();


        while *sender_running.lock().unwrap() {
            print!("You:"); // TODO fix me
            stdout.flush().unwrap(); // TODO unwrap

            input_str.clear(); // clear old input each iter

            if stdin.read_line(&mut input_str).is_err() {
                println!("oopsies could not read that line");
            }

            let mut stream = sending_stream.lock().unwrap();

            if let Err(e) = stream.write_all(input_str.as_bytes()){
                println!("Error could not send data {}", e);
                break;
            }

        
        }

    });


    rec_thread.join().unwrap();
    send_thread.join().unwrap();


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

    match input_string.trim() {
        "" => Ok(SocketAddr {
            ip_addr: IP_Address("127.0.0.1".to_string()),
            port: Port(34254),
        }),
        _ => input_string.trim().parse::<rustcp::SocketAddr>(),
    }
}

fn open_stream(socket: SocketAddr) -> Result<TcpStream, RustChatError> {
    let bind_addr: String = socket.ip_addr.0 + ":" + socket.port.0.to_string().as_str();

    let stream = TcpStream::connect(bind_addr)
        .map_err(|_| RustChatError::TcpStreamError("Could not open TCP stream".to_string()));

    return stream;
}

