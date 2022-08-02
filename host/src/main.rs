mod factory;

use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use clap::Parser;

use utils::*;
use traits::*;
use factory::*;
use cli::*;

fn main() {
    let args = Cli::parse();

    if args.download_uri.is_some() && args.download_name.is_some() {
        println!("Downloading...");
    }
    
    if !args.host {
        return;
    }

    let url: String = format!("localhost:{}", args.port);

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
    if req.path == "add" {
        return responder.generate_post_response(&req, &fs)
    }
    responder.generate_get_response(&req, &fs)
}