use std::io::prelude::*;
use std::fs::File;

pub fn get_file_buff(fname: &String) -> Vec<u8> { 
    let mut buff = Vec::new();
    println!("Filename: {}", fname);
    let mut file = File::open(fname).unwrap();
    file.read_to_end(&mut buff).unwrap();
    buff
}