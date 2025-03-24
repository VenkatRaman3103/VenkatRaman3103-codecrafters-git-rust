use crate::fs_utils;
use flate2::{read::ZlibDecoder, write::ZlibEncoder, Compression};
use hex;
use sha1::{Digest, Sha1};
use std::io::Read;
use std::{fs::File, io::Write};

pub fn read_blob(argument: Vec<String>) {
    let blob_object = &argument[3];

    let object_folder = &blob_object[0..2];

    let object_file = format!(".git/objects/{}/{}", object_folder, &blob_object[2..]);

    let bytes = fs_utils::read_file(&object_file);

    let mut deflater = ZlibDecoder::new(&bytes[..]);
    let mut file_content = String::new();
    deflater.read_to_string(&mut file_content).unwrap();

    let body = file_content
        .split_once('\x00')
        .expect("Error in splitting")
        .1;

    print!("{}", body);
}
pub fn create_blob(argument: &str) {
    // 1. Read the content of the file
    let file_content_bytes = fs_utils::read_file(argument);

    // 2. Create blob header
    let head = format!("blob {}\x00", file_content_bytes.len());

    // 3. Concatenate header and body
    let mut blob_content = Vec::new();
    blob_content.extend_from_slice(head.as_bytes());
    blob_content.extend_from_slice(&file_content_bytes);

    // 4. Compute SHA-1 hash
    let mut hasher = Sha1::new();
    hasher.update(&blob_content);
    let object_hash = hasher.finalize();
    let hex_hash = hex::encode(object_hash);

    println!("{}", hex_hash);

    // 5. Compress blob content
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder
        .write_all(&blob_content)
        .expect("Error compressing blob");
    let compressed_data = encoder.finish().expect("Error finalizing compression");

    // 6. Prepare object storage path
    let object_folder = format!(".git/objects/{}", &hex_hash[0..2]);
    let object_file = format!("{}/{}", object_folder, &hex_hash[2..]);

    // 7. Create folder if not exists
    fs_utils::create_folder(&object_folder).expect("Error creating folder");

    // 8. Write compressed data to the object file
    let mut blob_file = File::create(&object_file).expect("Error creating blob file");
    blob_file
        .write_all(&compressed_data)
        .expect("Error writing blob");
}
