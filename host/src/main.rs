mod factory;
use std::io::prelude::*;
use std::io::BufReader;
use std::net::TcpListener;
use std::net::TcpStream;
use utils::*;
use traits::*;
use factory::*;

fn main() {
    let host = TcpListener::bind("localhost:7878").unwrap();

    let factory = Factory {};

    for stream in host.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream, &factory);
    }
}

fn handle_connection(mut stream: TcpStream, factory: &Factory) {
    let parser = factory.make_parser();

    let mut reader = BufReader::new(&stream);
    println!("stream: {:?}", stream);

    let req: Request = parser.get_req(&stream);

    let response = get_response(&req, &factory);
    
    stream.write(&response).unwrap();
    stream.flush().unwrap();
}

fn get_response(req: &Request, factory: &Factory) -> Vec<u8> {
    let responder = factory.make_responder();
    let fs = factory.make_filesys();
    if req.path == "add" {
        return responder.generate_post_response(&req, &fs)
    }
    responder.generate_get_response(&req, &fs)
}