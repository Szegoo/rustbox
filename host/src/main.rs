use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use filesys;
use reqparser;

fn main() {
    let host = TcpListener::bind("172.20.14.11:7878").unwrap();

    for stream in host.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let req = format!("{:?}", String::from_utf8_lossy(&buffer[..]));
    let path = reqparser::get_url_path(&req);
    let extension = reqparser::get_file_extension(&path);

    if path == "favicon.ico" {
        return;
    } 

    let buff = filesys::get_file_buff(path);
    let headers = reqparser::get_headers(extension);
    let response = generate_response(headers, buff);
    
    stream.write(&response).unwrap();
    stream.flush().unwrap();
}

fn generate_response(headers: [String; 3], buff: Vec<u8>) -> Vec<u8> {
    let mut response = headers.join("\r\n")
        .to_string()
        .into_bytes();
    response.extend(buff);
    response
}