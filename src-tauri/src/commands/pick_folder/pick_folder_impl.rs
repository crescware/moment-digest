use std::path::PathBuf;
use futures::channel::oneshot;
use tauri::api::dialog::FileDialogBuilder;

pub async fn pick_folder_impl() -> Option<PathBuf> {
    let (tx, rx) = oneshot::channel();
    FileDialogBuilder::new().pick_folder(|path| {
        tx.send(path).unwrap();
    });

    rx.await.unwrap()
}
