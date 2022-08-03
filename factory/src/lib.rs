use traits::FactoryT;

type Reqparser = reqparser::Reqparser;
type Responder = responder::Responder;
type FileSys = filesys::FileSys;
type Backer = backer::Backer;

pub struct Factory;

impl FactoryT for Factory {
    type Parser = Reqparser;
    type Responder = Responder;
    type FileSys = FileSys;
    type Backer = Backer;

    fn make_parser(&self) -> Reqparser {
        Reqparser {}
    }

    fn make_responder(&self) -> Responder {
        Responder {}
    }

    fn make_filesys(&self) -> FileSys {
        FileSys {}
    }

    fn make_backer(&self) -> Backer {
        Backer {}
    }
}
