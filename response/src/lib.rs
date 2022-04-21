use std::panic;
use filesys;
use utils;

pub fn generate_response(req: &utils::Request) -> Vec<u8> {
    let res_maybe = panic::catch_unwind(|| {
        try_generate(&req)
    });
    let res = match res_maybe {
        Ok(d) => d,
        Err(_) => get_err_response().join("\r\n")
        .to_string()
        .into_bytes()
    }; 
    res
}

fn try_generate(req: &utils::Request) -> Vec<u8> {
    let buff = filesys::get_file_buff(&req.path);

    let extension = get_file_extension(&req.path);
    let headers = get_headers(extension);

    let mut response = headers.join("\r\n")
        .to_string()
        .into_bytes();
    response.extend(buff);
    response
}

fn get_file_extension(path: &String) -> String {
    let dot_indx = path.find(".").unwrap();
    path.chars().skip(dot_indx+1).collect()
}

fn get_headers(extension: String) -> [String; 3] {
    let ctype = get_req_ctype(&extension); 
    let content = format!("Content-Type: {}/{}", ctype, extension);
    let headers = [
        String::from("HTTP/1.1 200 OK"),
        content,
        String::from("\r\n")
    ];
    headers
}

fn get_req_ctype(extension: &String) -> String {
    match extension.as_str() {
        "pdf" => String::from("application"),
        "jpeg" | "jpg" | "png" => String::from("image"),
        _ => panic!("Unknown content type"), 
    }
}

fn get_err_response() -> [String; 3] {
    let headers = [
        String::from("HTTP/1.1 404 Not found"),
        String::from("Content-Type: Text/html"),
        String::from("\r\n")
    ];
    headers
}