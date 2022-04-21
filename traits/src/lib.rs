pub trait Reqparser {
    type Stream; 

    fn get_req(stream: &Self::Stream) -> utils::Request;
}