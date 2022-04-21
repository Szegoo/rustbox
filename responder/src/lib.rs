use std::panic;
use filesys;
use utils;
use traits;

pub struct Responder;

impl traits::Responder for Responder {
    fn generate_post_response(req: &utils::Request) -> Vec<u8> {
        //TODO currentnly this just returns the headers <- fix that
        Self::try_generate_post()
    }

    fn generate_get_response(req: &utils::Request) -> Vec<u8> {
        let res_maybe = panic::catch_unwind(|| {
            Self::try_generate_get(&req)
        });
        let res = match res_maybe {
            Ok(d) => d,
            Err(_) => Self::get_err_response().join("\r\n")
            .to_string()
            .into_bytes()
        }; 
        res
    }
}

impl Responder {
    fn try_generate_post() -> Vec<u8> {
        Self::get_post_headers().join("\r\n")
            .to_string()
            .into_bytes()
    }

    fn try_generate_get(req: &utils::Request) -> Vec<u8> {
        let buff = filesys::get_file_buff(&req.path);

        let extension = Self::get_file_extension(&req.path);
        let headers = Self::get_headers(extension);

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
        let ctype = Self::get_req_ctype(&extension); 
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

    fn get_post_headers() -> [String; 3] {
        let headers = [
            String::from("HTTP/1.1 201 File saved"),
            String::from("Content-Type: Text/html"),
            String::from("\r\n")
        ];
        headers
    }
}