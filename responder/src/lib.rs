use std::panic;
use utils::*;
use traits::*;

pub struct Responder;

impl traits::Responder for Responder {
    fn generate_post_response<T: FileSys>(&self, req: &Request, fs: &T) -> Vec<u8> {
        //TODO currentnly this just returns the headers <- fix that
        Self::try_generate_post()
    }

    fn generate_get_response<T: FileSys>(&self, req: &Request, fs: &T) -> Vec<u8> {
        Self::try_generate_get(&req, fs)
    }
}

impl Responder {
    fn try_generate_post() -> Vec<u8> {
        Self::get_post_headers().join("\r\n")
            .to_string()
            .into_bytes()
    }

    fn try_generate_get<T: FileSys>(req: &Request, fs: &T) -> Vec<u8> {
        let buff = fs.get_file_buff(&req.path);
        let res = match buff {
            Ok(buff) => Self::get_res_with_buff(req, &buff),
            Err(_) => Self::get_err_res()
        };

        res
    }

    fn get_res_with_buff(req: &Request, buff: &Vec<u8>) -> Vec<u8> {
        let extension = Self::get_file_extension(&req.path);
        let headers = Self::get_headers(extension);

        let mut response = headers.join("\r\n")
            .to_string()
            .into_bytes();
        response.extend(buff);
        response
    }

    fn get_err_res() -> Vec<u8> {
        Self::get_err_response().join("\r\n")
           .to_string()
           .into_bytes()
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