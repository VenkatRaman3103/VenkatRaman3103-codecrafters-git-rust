use std::env;

mod blob;
mod fs_utils;
mod initialize;
mod tree;

fn main() {
    let argument: Vec<String> = env::args().collect();
    // let root = ".git";
    let root = "test/.git";

    if argument.len() > 1 && argument[1] == "init" {
        initialize::git_repo(root);
        return;
    }

    if argument.len() > 3 && argument[1] == "cat-file" && argument[2] == "-p" {
        let blob_object = &argument[3];

        blob::read_blob(blob_object);
        return;
    }

    if argument.len() > 3 && argument[1] == "hash-object" && argument[2] == "-w" {
        blob::create_blob(&argument[3]);
        return;
    }

    if argument.len() > 3 && argument[1] == "ls-tree" && argument[2] == "--name-only" {
        let tree_sha = &argument[3];
        tree::read_tree(tree_sha);
        return;
    }

    if argument.len() > 3 && argument[1] == "create-blob" && argument[2] == "tree" {
        let blob_type = &argument[2];
        let blob_sha = &argument[3];

        println!("{}", blob_sha);

        let created_path = fs_utils::create_blob(root, blob_type, blob_sha);

        println!("{:?}", created_path);
        return;
    }

    println!("Incorrect argument");
}
