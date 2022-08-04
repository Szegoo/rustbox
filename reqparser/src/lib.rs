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
    fn get_url_path(&self, req: &str) -> String {
        // plus one becuase we don't need the '/'.
        let path_start_indx = req.find("/").unwrap() + 1;
        let cropped_req: String = req.chars().skip(path_start_indx).collect();

        let (path, _) = cropped_req
            .split_once(' ')
            .expect("Something is wrong with the request.");
        String::from(path)
    }
}
