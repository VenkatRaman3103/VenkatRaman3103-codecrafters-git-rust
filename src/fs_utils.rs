use std::fs;

// pub fn get_absolut_path(path: &str) -> Result<PathBuf, std::io::Error> {
//     fs::canonicalize(path)
// }

pub fn create_folder(path: &str) -> Result<(), std::io::Error> {
    fs::create_dir(path)
}
