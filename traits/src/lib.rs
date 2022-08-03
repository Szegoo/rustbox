use std::net::TcpStream;
use utils::*;

pub trait Reqparser {
    fn get_req(&self, stream: &TcpStream) -> utils::Request;
}

pub trait Responder {
    fn generate_response<T: FileSys>(&self, req: &Request, fs: &T) -> Vec<u8>;
}

pub trait FileSys {
    fn get_file_buff(&self, fname: &String) -> Result<Vec<u8>, u8>;
}

pub trait Backer {
    fn download_from(&self, uri: &String, fname: &String);
}

pub trait FactoryT {
    type Parser;
    type Responder;
    type FileSys;
    type Backer;

    fn make_parser(&self) -> Self::Parser;
    fn make_responder(&self) -> Self::Responder;
    fn make_filesys(&self) -> Self::FileSys;
    fn make_backer(&self) -> Self::Backer;
}
