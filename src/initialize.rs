use std::{fs::File, io::Write};

use crate::fs_utils;

pub fn git_repo(root: &str) {
    println!("Initializing Git Reposistory");

    fs_utils::create_folder(root).expect("Error in creating root folder");

    let list_of_folders: [&str; 2] = ["/objects", "/refs"];

    for folder in list_of_folders {
        let folder_path = format!("{}{}", root, folder);
        let error_message = format!("Error in creating folder: {}", folder_path);

        fs_utils::create_folder(&folder_path).expect(&error_message);

        println!("{}", folder);
    }

    // create a file
    let file_name = format!("{}/HEAD", root);

    let mut file = File::create(file_name).expect("Error in creating file");

    file.write_all(b"ref: refs/heads/main\n")
        .expect("Error in writting data");
}
