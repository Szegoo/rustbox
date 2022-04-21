use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use response;
use utils;
use reqparser::Reqparser;

pub type Stream = TcpStream;

fn main() {
    let host = TcpListener::bind("192.168.1.11:7878").unwrap();

    for stream in host.incoming() {
        let stream = stream.unwrap();
        handle_connection::<reqparser::Reqparser>(stream);
    }
}

fn handle_connection<Reqparser: traits::Reqparser<Stream = Stream>>(mut stream: TcpStream) {
    let req: utils::Request = Reqparser::get_req(&stream);
    let response = response::generate_response(&req);
    
    stream.write(&response);
    stream.flush().unwrap();
}