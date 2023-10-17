// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::api::dialog::FileDialogBuilder;

#[tauri::command]
fn pick_folder() {
    FileDialogBuilder::new().pick_folder(|path| {
        println!("Selected path: {:?}", path);
    })
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![pick_folder])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
