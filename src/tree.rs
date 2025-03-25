use crate::fs_utils;

pub fn read_tree(tree_sha: &str) {
    let blob_file_path = format!("test/.git/objects/{}/{}", &tree_sha[..2], &tree_sha[2..]);

    let bytes = fs_utils::read_file(&blob_file_path);

    println!("{}", hex::encode(bytes));
}
