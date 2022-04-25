use std::net::TcpStream;

pub trait Reqparser {
    fn get_req(&self, stream: &TcpStream) -> utils::Request;
}

pub trait Responder {
    fn generate_post_response(&self, req: &utils::Request) -> Vec<u8>;
    fn generate_get_response(&self, req: &utils::Request) -> Vec<u8>;
}

pub trait FileSys {
    fn get_file_buff(&self, fname: &String) -> Vec<u8>;
    fn save_file(&self, fname: &String);
}
