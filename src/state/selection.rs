use super::{AppState, to_rel_path_string};
use crate::preview::{FilePreview, PreviewContent, load_preview};
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
        if idx < self.selected_files.len() {
            let removed = self.selected_files.remove(idx);
            let rel = to_rel_path_string(self.project_root.as_ref(), &removed);
            self.status = format!("Removed file: {}", rel);

            if self.selected_files.is_empty() {
                self.active_list_index = None;
                self.preview_path = None;
                self.preview_content = None;
            } else {
                let new_idx = if idx >= self.selected_files.len() {
                    self.selected_files.len() - 1
                } else {
                    idx
                };
                self.select_export_file(new_idx);
            }
        }
    }

    pub fn load_preview(&mut self, path: PathBuf) {
        self.preview_path = Some(path.clone());
        match load_preview(&path) {
            Ok(preview) => {
                let size = preview.size;
                let rel_path = self
                    .project_root
                    .as_ref()
                    .and_then(|root| path.strip_prefix(root).ok())
                    .unwrap_or(&path)
                    .display()
                    .to_string();
                self.status = format!("Loaded {} bytes: {}", size, rel_path);
                self.preview_content = Some(preview);
            }
            Err(e) => {
                self.preview_content = Some(FilePreview {
                    path: path.clone(),
                    size: 0,
                    content: PreviewContent::Error(e.to_string()),
                });
                self.status = format!("Error loading preview: {}", e);
            }
        }
    }
}
