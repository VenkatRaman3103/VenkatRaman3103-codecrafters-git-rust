use std::{fs, path::PathBuf};

pub fn get_absolut_path(path: &str) -> Result<PathBuf, std::io::Error> {
    fs::canonicalize(path)
}
