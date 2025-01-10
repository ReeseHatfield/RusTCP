use std::{io::{stdin, Read, Write}, net::TcpStream, vec};

use RusTCP::rustcp;

fn main() -> Result<(), rustcp::RustChatError>{

    // let socket = "127.0.0.1:34254";

    let socket = read_socket()?;
    // stream -> connection from client and server
    // stream will get closed whenever drop(stream) is called

    println!("Read socket as {:?}", socket);


    // let mut stream = match TcpStream::connect(socket) {
    //     Ok(stream) => stream,
    //     Err(_) => {
    //         println!("Could not open TCP stream at socket {:?}", socket);
    //         return;
    //     }
    // };


    // let message = "Here is some wierd other data";

    // stream.write(message.as_bytes()).unwrap();
    // stream.flush().unwrap();

    // println!("I wrote {:?}", message);

    // let mut buf: rustcp::Buffer = vec![0; 1024];
    // stream.read(&mut buf).unwrap();

    // // println!("BUF: {:?}", buf);

    // if let Ok(res) = rustcp::buf_to_string(&buf){
    //     println!("I got back: {:?}", res);
    // }



    // println!("TCP stream: {:?}", stream);

    Ok(())

}


fn read_socket() -> Result<rustcp::Socket, rustcp::RustChatError> {
    let mut input_string = String::new();
    println!("Please enter the socket in the form IP:PORT");

    // wanna match on the input here 

    if stdin().read_line(&mut input_string).is_err(){
        return Err(rustcp::RustChatError::SocketParseError("Could not read socket from user".to_owned()));
    }

    return input_string.trim().parse::<rustcp::Socket>();

}

