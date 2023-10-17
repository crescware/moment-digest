mod pick_folder_impl;

use std::fs;
use std::path::{Path, PathBuf};
use std::io;
use regex::Regex;

fn list_files_in_directory<P: AsRef<Path>>(dir: P) -> io::Result<Vec<PathBuf>> {
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

#[tauri::command]
pub async fn pick_folder() {
    let path = pick_folder_impl::pick_folder_impl().await.unwrap();
    println!("Selected path: {:?}", path);

    println!("list_files_in_director");

    let files = list_files_in_directory(path).unwrap();
    println!("Files: {:?}", files);
}
