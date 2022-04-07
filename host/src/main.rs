use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs::File;

fn main() {
    let host = TcpListener::bind("192.168.1.11:7878").unwrap();

    for stream in host.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let req = format!("{:?}", String::from_utf8_lossy(&buffer[..]));
    let path = get_url_path(&req);
    let fext = get_file_extension(&path);

    if path == "favicon.ico" {
        return;
    } 

    let buff = get_file_buff(path);
    let headers = get_headers(fext);
    let response = generate_response(headers, buff);
    
    stream.write(&response).unwrap();
    stream.flush().unwrap();
}

fn get_url_path(req: &str) -> String {
    let slash_indx = req.find("/").unwrap();
    let new_req: String = req.chars().skip(slash_indx+1).collect();
    let slash2_indx = new_req.find("/").unwrap();
    let path: String = new_req.chars().take(slash2_indx - slash_indx).collect();
    path
}

fn get_file_extension(path: &String) -> String {
    let dot_indx = path.find(".").unwrap();
    path.chars().skip(dot_indx+1).collect()
}

fn get_file_buff(fname: String) -> Vec<u8> { 
    let mut buff = Vec::new();
    println!("Filename: {}", fname);
    let mut file = File::open(fname).unwrap();
    file.read_to_end(&mut buff).unwrap();
    buff
}

fn get_headers(fext: String) -> [String; 3] {
    let ctype = get_file_ctype(&fext); 
    let content = format!("Content-Type: {}/{}", ctype, fext);
    let headers = [
        String::from("HTTP/1.1 200 OK"),
        content,
        String::from("\r\n")
    ];
    headers
}

fn get_file_ctype(path: &String) -> String {
    match path.as_str() {
        "pdf" => String::from("application"),
        "jpeg" | "jpg" | "png" => String::from("image"),
        _ => panic!("Unknown content type"), 
    }
}

fn generate_response(headers: [String; 3], buff: Vec<u8>) -> Vec<u8> {
    let mut response = headers.join("\r\n")
        .to_string()
        .into_bytes();
    response.extend(buff);
    response
}