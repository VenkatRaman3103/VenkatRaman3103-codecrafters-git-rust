use std::{env, fs::File, io::Write};

mod fs_utils;

fn main() {
    let argument: Vec<String> = env::args().collect();

    if argument.len() > 1 && argument[1] == "init" {
        println!("Initializing Git Reposistory");

        fs_utils::create_folder("test").expect("Error in creating root folder");

        let root = "test/.git";

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
    } else {
        println!("Incorrect argument")
    }
}
