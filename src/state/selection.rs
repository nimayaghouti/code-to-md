use super::{AppState, to_rel_path_string};
use std::path::PathBuf;

impl AppState {
    pub fn select_tree_file(&mut self, path: PathBuf) {
        self.selected_tree_file = Some(path.clone());
        self.active_list_index = None;
        self.load_preview(path);
    }

    pub fn select_export_file(&mut self, idx: usize) {
        if idx < self.selected_files.len() {
            self.active_list_index = Some(idx);
            self.selected_tree_file = None;
            let path = self.selected_files[idx].clone();
            self.load_preview(path);
        }
    }

    pub fn add_file_to_export(&mut self, path: PathBuf) {
        if path.is_dir() {
            return;
        }

        if self.selected_files.contains(&path) {
            let rel = to_rel_path_string(self.project_root.as_ref(), &path);
            self.status = format!("File already in export list: {}", rel);
            return;
        }

        let rel = to_rel_path_string(self.project_root.as_ref(), &path);
        self.selected_files.push(path);
        self.status = format!("Added file: {}", rel);
    }

    pub fn remove_selected_from_export(&mut self) {
        if let Some(idx) = self.active_list_index {
            self.remove_file_by_index(idx);
        }
    }

    pub fn remove_file_by_index(&mut self, idx: usize) {
        if idx >= self.selected_files.len() {
            return;
        }

        let removed = self.selected_files.remove(idx);

        let rel = to_rel_path_string(self.project_root.as_ref(), &removed);

        self.status = format!("Removed file: {}", rel);

        if self.selected_files.is_empty() {
            self.active_list_index = None;
            self.preview_path = None;
            self.preview_content = None;
            self.clear_preview_cache();
            return;
        }

        let new_idx = if idx >= self.selected_files.len() {
            self.selected_files.len() - 1
        } else {
            idx
        };
        self.select_export_file(new_idx);
    }
}
