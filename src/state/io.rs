use super::AppState;
use crate::fs_tree::build_tree;
use rfd::FileDialog;
use std::path::PathBuf;

impl AppState {
    pub fn open_folder_dialog(&mut self) {
        if let Some(path) = FileDialog::new().pick_folder() {
            self.open_folder(path);
        }
    }

    pub fn open_folder(&mut self, path: PathBuf) {
        match build_tree(&path) {
            Ok(tree) => {
                self.project_root = Some(path.clone());
                self.file_tree = Some(tree);
                self.selected_tree_file = None;
                self.preview_path = None;
                self.preview_content = None;
                self.selected_files.clear();
                self.active_list_index = None;
                self.status = format!("Opened folder: {}", path.display());
            }
            Err(e) => {
                self.status = format!("Error opening folder: {}", e);
            }
        }
    }

    pub fn select_output_path_dialog(&mut self) -> Option<PathBuf> {
        let mut dialog = FileDialog::new()
            .set_file_name("project-files.md")
            .add_filter("Markdown (*.md)", &["md"]);

        if let Some(ref current) = self.output_path {
            if let Some(parent) = current.parent() {
                dialog = dialog.set_directory(parent);
            }
        } else if let Some(desktop) = dirs::desktop_dir() {
            dialog = dialog.set_directory(desktop);
        }

        if let Some(path) = dialog.save_file() {
            self.output_path = Some(path.clone());
            self.status = format!("Set output file: {}", path.display());
            Some(path)
        } else {
            None
        }
    }
}
