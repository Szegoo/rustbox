use std::fs::File;
use std::io;

pub struct Backer;

impl traits::Backer for Backer {

    /// Downloads the requested file from the defined `uri` and saves it into a new file named
    /// `fname`.
    ///
    /// Couple of notes: For now the `fname` needs to contain the file extension but this will be
    /// automatically inferred in the future. Also in the future there should be a check to make
    /// sure we don't override existing files.
    fn download_from(&self, uri: &String, fname: &String) {
        let mut resp =
            reqwest::blocking::get(uri).expect("An error occurred while downloading the file.");

        let mut out = File::create(fname).expect("An error occurred during file creation.");
        io::copy(&mut resp, &mut out).expect("Failed to write into the file");
    }
}
