use std::fs::File;
use std::io;

pub struct Backer;

impl traits::Backer for Backer {
    /// Downloads the requested file from the defined `uri` and saves it into a new file named
    /// `fname`.
    ///
    /// NOTE: In the future there should be a check to make sure we don't override existing files.
    fn download_from(&self, uri: &String, fname: &String) {
        let ext = Self::get_file_extension(uri);
        let fname_with_ext = format!("{}.{}", fname, ext);
        let mut resp =
            reqwest::blocking::get(uri).expect("An error occurred while downloading the file.");

        let mut out =
            File::create(fname_with_ext).expect("An error occurred during file creation.");
        io::copy(&mut resp, &mut out).expect("Failed to write into the file");
    }
}

impl Backer {
    /// returns the extension of a file without the '.'
    fn get_file_extension(uri: &String) -> String {
        let (_, ext) = uri
            .rsplit_once('.')
            .expect("Couldn't detect the file extension from the uri.");
        String::from(ext)
    }
}
