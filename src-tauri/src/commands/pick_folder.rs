mod pick_folder_impl;

use regex::Regex;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use tauri::api::dialog::FileDialogBuilder;

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

struct FileDialogBuilderProxy(FileDialogBuilder);

impl pick_folder_impl::FileDialogBuilderTrait for FileDialogBuilderProxy {
    fn pick_folder(self, f: Box<dyn FnOnce(Option<PathBuf>) + Send + 'static>) {
        self.0.pick_folder(f)
    }
}

#[tauri::command]
pub async fn pick_folder() {
    FileDialogBuilder::new().pick_folder(Box::new(|path| {
        println!("Selected path: {:?}", path);
    }));


    let builder = FileDialogBuilderProxy(FileDialogBuilder::new());
    let path = pick_folder_impl::pick_folder_impl(builder).await.unwrap();
    println!("Selected path: {:?}", path);

    println!("list_files_in_director");

    let files = list_files_in_directory(path).unwrap();
    println!("Files: {:?}", files);
}
