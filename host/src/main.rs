use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use response;

fn main() {
    let host = TcpListener::bind("172.20.14.11:7878").unwrap();

    for stream in host.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let response = response::generate_response(&stream);
    
    stream.write(&response);
    stream.flush().unwrap();
}