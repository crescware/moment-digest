mod file_dialog_builder_trait;
mod list_files_in_directory;
mod pick_folder_impl;

use file_dialog_builder_trait::FileDialogBuilderProxy;
use list_files_in_directory::list_files_in_directory;
use pick_folder_impl::pick_folder_impl;
use tauri::api::dialog::FileDialogBuilder;

#[tauri::command]
pub async fn pick_folder() {
    let builder = FileDialogBuilderProxy::new(FileDialogBuilder::new());
    let path = pick_folder_impl(builder).await.unwrap();
    println!("Selected path: {:?}", path);

    println!("list_files_in_director");

    let files = list_files_in_directory(path).unwrap();
    println!("Files: {:?}", files);
}
