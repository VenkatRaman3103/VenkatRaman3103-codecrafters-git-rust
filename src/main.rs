use std::env;

mod blob;
mod fs_utils;
mod initialize;

fn main() {
    let argument: Vec<String> = env::args().collect();
    let root = ".git";
    // let root = "test/.git";

    if argument.len() > 1 && argument[1] == "init" {
        initialize::git_repo(root);
        return;
    }

    if argument.len() > 3 && argument[1] == "cat-file" && argument[2] == "-p" {
        blob::read_blob(argument);
        return;
    }

    if argument.len() > 3 && argument[1] == "hash-object" && argument[2] == "-w" {
        blob::create_blob(&argument[3]);
        return;
    }

    println!("Incorrect argument");
}
