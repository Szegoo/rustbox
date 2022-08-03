use traits;

pub struct Backer;

impl traits::Backer for Backer {
    fn download_from(&self, uri: &String, fname: &String) {
        let res = reqwest::blocking::get(uri)
            .expect("An error occurred while downloading the file.")
            .bytes();
    }
}
