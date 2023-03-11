use std::io::{Read, Write};
use std::net::TcpListener;

fn main() {
    let connection_listener = TcpListener::bind("127.0.0.1:3000").expect("Unable to bind");
    println!("RUNNING on PORT 3000");

    for stream in connection_listener.incoming() {
        let mut stream = stream.unwrap();
        println!("Connection established");
        let mut buffer = [0; 100];
        stream.read(&mut buffer).unwrap();
        println!("Received from client: {}", String::from_utf8_lossy(&buffer));
        stream.write(&mut buffer).unwrap();
    }
}
