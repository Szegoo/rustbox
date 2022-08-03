use std::panic;
use traits::*;
use utils::*;

pub struct Responder;

impl traits::Responder for Responder {
    /// Generates the response for the given `req`.
    fn generate_response<T: FileSys>(&self, req: &Request, fs: &T) -> Vec<u8> {
        self.try_generate_get(&req, fs)
    }
}

impl Responder {
    /// If the file exists returns the file, otherwise returns an error response.
    fn try_generate_get<T: FileSys>(&self, req: &Request, fs: &T) -> Vec<u8> {
        if let Ok(buff) = fs.get_file_buff(&req.path) {
            return self.generate_res_with_buff(req, &buff);
        }

        self.get_err_res()
    }

    /// Generates an http response with the given buffer.
    fn generate_res_with_buff(&self, req: &Request, buff: &Vec<u8>) -> Vec<u8> {
        let extension = self.get_file_extension(&req.path);
        let headers = self.get_headers(extension);

        let mut response = headers.join("\r\n").to_string().into_bytes();
        response.extend(buff);
        response
    }

    /// Returns the error HTTP response.
    fn get_err_res(&self) -> Vec<u8> {
        self.get_err_response()
            .join("\r\n")
            .to_string()
            .into_bytes()
    }

    /// Gets the file extension of the requested file.
    fn get_file_extension(&self, path: &String) -> String {
        let dot_indx = path.find(".").unwrap();
        path.chars().skip(dot_indx + 1).collect()
    }

    /// Returns the headers for a successful HTTP response.
    fn get_headers(&self, extension: String) -> [String; 3] {
        let ctype = self.get_req_ctype(&extension);
        let content = format!("Content-Type: {}/{}", ctype, extension);
        let headers = [
            String::from("HTTP/1.1 200 OK"),
            content,
            String::from("\r\n"),
        ];
        headers
    }

    /// Returns the content-type HTTP field based on the extension.
    fn get_req_ctype(&self, extension: &String) -> String {
        match extension.as_str() {
            "pdf" => String::from("application"),
            "jpeg" | "jpg" | "png" => String::from("image"),
            _ => panic!("Unknown content type"),
        }
    }

    /// Returns the error HTTP response, currently this just returns a 404 Not found page.
    ///
    /// NOTE: In future this should be expanded to support more file extensions.
    fn get_err_response(&self) -> [String; 3] {
        let headers = [
            String::from("HTTP/1.1 404 Not found"),
            String::from("Content-Type: Text/html"),
            String::from("\r\n"),
        ];
        headers
    }
}
