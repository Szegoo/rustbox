use std::fs::File;
use std::io::prelude::*;
use traits;

pub struct FileSys;

impl traits::FileSys for FileSys {
    fn get_file_buff(&self, fname: &String) -> Result<Vec<u8>, u8> {
        let mut buff = Vec::new();
        println!("Filename: {}", fname);
        if let Ok(mut file) = File::open(fname) {
            file.read_to_end(&mut buff).expect("buffer overflow!");
            return Ok(buff);
        }

        return Err(1);
    }

    fn save_file(&self, fname: &String) {}
}
