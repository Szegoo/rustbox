use filesys;
use reqparser;
use responder;

type Reqparser = reqparser::Reqparser;
type Responder = responder::Responder;
type FileSys = filesys::FileSys;

pub trait FactoryT {
    type Parser;
    type Responder;
    type FileSys;

    fn make_parser(&self) -> Self::Parser;
    fn make_responder(&self) -> Self::Responder; 
    fn make_filesys(&self) -> Self::FileSys;
}

pub struct Factory;

impl FactoryT for Factory {
    type Parser = Reqparser;
    type Responder = Responder;
    type FileSys = FileSys;

    fn make_parser(&self) -> Reqparser {
        Reqparser {}
    }

    fn make_responder(&self) -> Responder {
        Responder {}
    }

    fn make_filesys(&self) -> FileSys {
        FileSys {}
    }
}