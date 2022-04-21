use std::net::TcpStream;
use std::io::prelude::*;
use utils;
use traits;

pub struct Reqparser;

impl Reqparser {
     fn get_req_buff(mut stream: &TcpStream) -> String {
        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap();

        format!("{:?}", String::from_utf8_lossy(&buffer[..]))
    }

    fn get_url_path(req: &str) -> String {
        let slash_indx = req.find("/").unwrap();
        let new_req: String = req.chars().skip(slash_indx+1).collect();
        let slash2_indx = new_req.find("/").unwrap();
        let path: String = new_req.chars().take(slash2_indx - slash_indx).collect();
        path
    }
}

impl traits::Reqparser for Reqparser{
    type Stream = TcpStream;
    
    fn get_req(stream: &Self::Stream) -> utils::ReqDTO {
        let req_buff = Self::get_req_buff(stream);
        let path = Self::get_url_path(&req_buff);
        
        utils::ReqDTO {
            req: req_buff,
            path,
        }
    }
}
