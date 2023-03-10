use  std::net::TcpListener;
use  std::net::TcpStream;
use std::io::prelude::*;

fn main() {
    let listener: TcpListener  = TcpListener::bind("127.0.0.1:8080").unwrap();

    for streamer in listener.incoming() {
        let stream = streamer.unwrap();

        println!("connection established!");
        Handle_connection(stream)
    }
}

fn Handle_connection(mut stream: TcpStream) {
    let mut buffer = [0;1024];
    stream.read(&mut buffer).unwrap();

    println!("Rquest {}", String::from_utf8_lossy(&buffer[..]))
}
