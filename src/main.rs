mod fs_utils;

// some
fn main() {
    let absolut_path = fs_utils::get_absolut_path("test/test.txt").unwrap();

    println!("{:?}", absolut_path);
}
