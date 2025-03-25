use std::{fs, io::Write};

use flate2::write::ZlibEncoder;
use flate2::Compression;

pub fn create_folder(path: &str) -> Result<(), std::io::Error> {
    fs::create_dir(path)
}

pub fn read_file(path: &str) -> Vec<u8> {
    fs::read(path).expect("Error in reading file")
}

// pub fn create_file(path: &str) -> Result<fs::File, std::io::Error> {
//     fs::File::create(path)
// }

pub fn open_file(path: &str) -> fs::File {
    fs::File::open(path).expect("Error in opening file")
    // fs::read(path).expect("Error in opening file")
}

pub fn create_blob(root: &str, blob_type: &str, path: &str) -> String {
    let object_folder_path = format!("{}/objects/{}", root, &path[0..2]);
    let object_file_path = format!("{}/{}", object_folder_path, &path[2..]);

    // create folder
    fs::create_dir(object_folder_path).expect("Error in creating blob folder");

    // create file
    let mut object_file = fs::File::create(&object_file_path).expect("Error in creating blob file");

    // write file
    if blob_type == "tree" {
        let tree_content = b"tree 123\0\
                            100644 blob file1.txt\0\
                            1234567890abcdef1234567890abcdef12345678\
                            100755 blob script.sh\0\
                            abcdef1234567890abcdef1234567890abcdef12\
                            040000 tree src\0\
                            7890abcdef1234567890abcdef1234567890abcdef12";

        let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());

        encoder
            .write_all(tree_content)
            .expect("Error compressing tree object");

        let compressed_data = encoder.finish().expect("Error finalizing compression");

        object_file
            .write_all(&compressed_data)
            .expect("Error in writing blob file");
    };

    object_file_path
}
