use  std::net::TcpListener;
use  std::net::TcpStream;
use std::io::prelude::*;
use std::fs;

fn main() {
    let listener  = TcpListener::bind("127.0.0.1:8080").expect("Oops can't secure a connection");

    for streamer in listener.incoming() {
        let stream = streamer.unwrap();

        println!("connection established!");
        Handle_connection(stream)
    }
}

fn Handle_connection(mut stream: TcpStream) {
    let mut buffer = [0;1024];
    stream.read(&mut buffer).unwrap();
    let get = b"GET / HTTP/1.1\r\n";
    if buffer.starts_with(get) {
        let content = fs::read_to_string("index.html").unwrap();

        let response = format!("HTTP/1.1 200 OK {}\r\n\r\n{}",content.len(),content);
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }else {
        let status_line = "HTTP/1.1 404 NOT FOUND";
        let content = fs::read_to_string("404.html").unwrap();

        let response = format!("{}\n\r Content-Length {}\r\n\r\n{}",status_line,content.len(),content);
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
    
}
