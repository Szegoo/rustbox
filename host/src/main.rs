mod factory;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use utils::*;
use traits::*;
use factory::*;

fn main() {
    let host = TcpListener::bind("192.168.1.11:7878").unwrap();

    let factory = Factory {};

    for stream in host.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream, &factory);
    }
}

fn handle_connection(mut stream: TcpStream, factory: &Factory) {
    let parser = factory.make_parser();
    let responder = factory.make_responder();
    let fs = factory.make_filesys();

    let req: Request = parser.get_req(&stream);
    if req.path == "/add" {
        return
    }
    let response = responder.generate_get_response(&req, &fs);
    
    stream.write(&response).unwrap();
    stream.flush().unwrap();
}