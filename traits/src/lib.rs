pub trait Reqparser {
    type Stream; 

    fn get_req(stream: &Self::Stream) -> utils::Request;
}

pub trait Responder {
    fn generate_post_response(req: &utils::Request) -> Vec<u8>;
    fn generate_get_response(req: &utils::Request) -> Vec<u8>;
}