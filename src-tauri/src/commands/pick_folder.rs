mod pick_folder_impl;

#[tauri::command]
pub async fn pick_folder() {
    let path = pick_folder_impl::pick_folder_impl().await.unwrap();
    println!("Selected path: {:?}", path);
}
