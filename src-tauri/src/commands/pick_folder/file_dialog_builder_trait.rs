use std::path::PathBuf;
use tauri::api::dialog::FileDialogBuilder;

type F = Box<dyn FnOnce(Option<PathBuf>) + Send + 'static>;

pub trait FileDialogBuilderTrait {
    fn pick_folder(self, f: F);
}

pub struct FileDialogBuilderProxy(FileDialogBuilder);

impl FileDialogBuilderProxy {
    pub fn new(v: FileDialogBuilder) -> FileDialogBuilderProxy {
        FileDialogBuilderProxy(v)
    }
}

impl FileDialogBuilderTrait for FileDialogBuilderProxy {
    fn pick_folder(self, f: F) {
        self.0.pick_folder(f)
    }
}
