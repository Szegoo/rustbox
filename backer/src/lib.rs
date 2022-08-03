use std::fs::File;
use std::io;

pub struct Backer;

impl traits::Backer for Backer {
    fn download_from(&self, uri: &String, fname: &String) {
        let mut resp =
            reqwest::blocking::get(uri).expect("An error occurred while downloading the file.");

        let mut out = File::create(fname).expect("An error occurred during file creation.");
        io::copy(&mut resp, &mut out).expect("Failed to write into the file");
    }
}
