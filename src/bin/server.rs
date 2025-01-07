use std::net::TcpListener;

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


    println!("Listening for TCP streams...");
    for stream in server.incoming(){
        // should pass to handle fn
        println!("Current incoming strem: {:?}", stream);
    }


}