use flate2::read::ZlibDecoder;
use std::env;
use std::fs;
use std::io::Read;

mod fs_utils;
mod initialize;

fn main() {
    let argument: Vec<String> = env::args().collect();
    let root = ".git";

    if argument.len() > 1 && argument[1] == "init" {
        initialize::git_repo(root);
        return;
    }

    if argument.len() > 3 && argument[1] == "cat-file" && argument[2] == "-p" {
        let blob_object = &argument[3];
        let object_folder = &blob_object[0..2];
        let object_file = format!(".git/objects/{}/{}", object_folder, &blob_object[2..]);

        let bytes = fs::read(object_file).expect("Error in reading file");

        let mut deflater = ZlibDecoder::new(&bytes[..]);
        let mut file_content = String::new();
        deflater.read_to_string(&mut file_content).unwrap();

        let body = file_content
            .split_once('\x00')
            .expect("Error in splitting")
            .1;

        print!("{}", body);
        return;
    }

    println!("Incorrect argument");
}
