use crate::fs_utils;
use std::io::Read;

use flate2::read::ZlibDecoder;

pub fn read_tree(root: &str, tree_sha: &str) {
    let tree_file_path = format!("{}/objects/{}/{}", &root, &tree_sha[..2], &tree_sha[2..]);

    let object_content = fs_utils::open_file(&tree_file_path);

    let decoder = ZlibDecoder::new(&object_content);

    let mut buf = String::new();

    let mut entries: Vec<String> = Vec::new();

    let mut space_found = false;
    let mut null_found = false;

    for byte in decoder.bytes() {
        let cursor = byte.unwrap();

        if cursor as char == ' ' {
            buf.clear();
            space_found = true;
        } else if cursor as char == '\x00' {
            if space_found && null_found {
                buf.push('\n');
                entries.push(buf.clone());
                buf.clear();
            } else {
                null_found = true;
            }
        } else {
            buf.push(cursor as char);
        }
    }

    print!("{}", entries.join(""));
}
