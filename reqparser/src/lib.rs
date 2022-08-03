use std::io::prelude::*;
use std::net::TcpStream;

pub struct Reqparser;

impl traits::Reqparser for Reqparser {
    /// Parses the `TcpStream` into `utils::Request`.
    fn get_req(&self, stream: &TcpStream) -> utils::Request {
        let buffer = self.get_req_buff(stream);
        let path = self.get_url_path(&buffer);

        utils::Request { buffer, path }
    }
}

impl Reqparser {
    /// Gets the buffer of the request.
    fn get_req_buff(&self, mut stream: &TcpStream) -> String {
        let mut buffer = [0; 512];
        stream.read(&mut buffer).unwrap();

        format!("{:?}", String::from_utf8_lossy(&buffer[..]))
    }

    /// Gets the path of the request. This is used to get the file the user is requesting.
    /// TODO this will need some cleanup, it looks pretty hacky now.
    fn get_url_path(&self, req: &str) -> String {
        let slash_indx = req.find("/").unwrap();
        let new_req: String = req.chars().skip(slash_indx + 1).collect();
        let slash2_indx = new_req.find("/").unwrap();
        let path: String;
        if slash2_indx > slash_indx {
            path = new_req.chars().take(slash2_indx - slash_indx).collect();
        } else {
            path = String::from("__none__");
        }
        path
    }
}
