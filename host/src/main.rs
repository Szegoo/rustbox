use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use responder;
use utils;

pub type Stream = TcpStream;

pub type Reqparser = reqparser::Reqparser;
pub type Responder = responder::Responder;

fn main() {
    let host = TcpListener::bind("192.168.1.11:7878").unwrap();

    for stream in host.incoming() {
        let stream = stream.unwrap();
        handle_connection::<Reqparser, Responder>(stream);
    }
}

fn handle_connection<Reqparser: traits::Reqparser<Stream = Stream>, Responder: traits::Responder>(mut stream: TcpStream) {
    let req: utils::Request = Reqparser::get_req(&stream);
    if req.path == "/add" {
        return
    }
    let response = Responder::generate_get_response(&req);
    
    stream.write(&response).unwrap();
    stream.flush().unwrap();
}