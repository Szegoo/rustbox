use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

use factory::Factory;
use traits::*;
use utils::*;

pub fn host(port: u16) {
    let url = format!("localhost:{}", port);
    let host = TcpListener::bind(url.clone()).unwrap();
    println!("Listening on: {}", url);

    let factory = Factory {};

    for stream in host.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream, &factory);
    }
}

fn handle_connection(mut stream: TcpStream, factory: &Factory) {
    let parser = factory.make_parser();

    println!("New connection!");

    let req: Request = parser.get_req(&stream);
    println!("Request: {:?}", req.buffer);

    let response = get_response(&req, &factory);

    stream.write(&response).unwrap();
    stream.flush().unwrap();
}

fn get_response(req: &Request, factory: &Factory) -> Vec<u8> {
    let responder = factory.make_responder();
    let fs = factory.make_filesys();

    responder.generate_get_response(&req, &fs)
}
