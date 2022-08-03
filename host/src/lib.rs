use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

use factory::Factory;
use traits::*;
use utils::*;

/// Starts listening on the defined `port`.
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

/// Handles the incoming connection and generates the response based on the request.
fn handle_connection(mut stream: TcpStream, factory: &Factory) {
    let parser = factory.make_parser();

    let req: Request = parser.get_req(&stream);
    println!("Request: {:?}", req.buffer);

    let response = generate_response(&req, &factory);

    stream.write(&response).unwrap();
    stream.flush().unwrap();
}

/// Generates the response based on the request.
fn generate_response(req: &Request, factory: &Factory) -> Vec<u8> {
    let responder = factory.make_responder();
    let fs = factory.make_filesys();

    responder.generate_response(&req, &fs)
}
