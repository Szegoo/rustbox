use std::io::prelude::*;
use std::fs::File;
use traits;

pub struct FileSys;

impl traits::FileSys for FileSys {
    fn get_file_buff(&self, fname: &String) -> Vec<u8> { 
        let mut buff = Vec::new();
        println!("Filename: {}", fname);
        let mut file = File::open(fname).unwrap();
        file.read_to_end(&mut buff).unwrap();
        buff
    }

    fn save_file(&self, fname: &String) {
        
    }
}
