use std::net::TcpStream;
use utils::*;

pub trait Reqparser {
    fn get_req(&self, stream: &TcpStream) -> utils::Request;
}

pub trait Responder {
    fn generate_post_response<T: FileSys>(&self, req: &Request, fs: &T) -> Vec<u8>;
    fn generate_get_response<T: FileSys>(&self, req: &Request, fs: &T) -> Vec<u8>;
}

pub trait FileSys {
    fn get_file_buff(&self, fname: &String) -> Result<Vec<u8>, u8>;
    fn save_file(&self, fname: &String);
}

pub trait Backer {
    fn download_from(&self, uri: &String, fname: &String);
}

pub trait FactoryT {
    type Parser;
    type Responder;
    type FileSys;

    fn make_parser(&self) -> Self::Parser;
    fn make_responder(&self) -> Self::Responder; 
    fn make_filesys(&self) -> Self::FileSys;
}