use regex::Regex;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

pub fn list_files_in_directory<P: AsRef<Path>>(dir: P) -> io::Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    let pattern = Regex::new(r"\d{6}\.md$").unwrap();

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && pattern.is_match(path.file_name().unwrap().to_str().unwrap()) {
            files.push(path);
        }
    }

    Ok(files)
}
