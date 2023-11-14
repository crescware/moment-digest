use futures::channel::oneshot;
use std::path::PathBuf;
use thiserror::Error;

pub trait FileDialogBuilderTrait {
    fn pick_folder(self, f: Box<dyn FnOnce(Option<PathBuf>) + Send + 'static>);
}

#[derive(Error, Debug)]
pub enum PickFolderImplError {
    #[error("The file dialog was closed without a selection")]
    DialogClosed,

    #[error("Failed to receive folder path")]
    FailedReceiveFolderPath,
}

pub async fn pick_folder_impl<T: FileDialogBuilderTrait>(
    builder: T,
) -> Result<PathBuf, PickFolderImplError> {
    let (tx, rx) = oneshot::channel();
    builder.pick_folder(Box::new(|path| {
        let _ = tx.send(path);
    }));

    match rx.await {
        Ok(Some(path)) => Ok(path),
        Ok(None) => Err(PickFolderImplError::DialogClosed),
        Err(_) => Err(PickFolderImplError::FailedReceiveFolderPath),
    }
}
