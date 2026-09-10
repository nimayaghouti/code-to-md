use super::{AppState, GitStatus};
use crate::fs_tree::build_tree;
use notify::RecursiveMode;
use notify_debouncer_mini::new_debouncer;
use rfd::FileDialog;
use std::path::PathBuf;
use std::time::Duration;

impl AppState {
    pub fn open_folder_dialog(&mut self) {
        if let Some(path) = FileDialog::new().pick_folder() {
            self.open_folder(path);
        }
    }

    pub fn open_folder(&mut self, path: PathBuf) {
        self.project_root = Some(path.clone());
        self.refresh_git_status();

        match build_tree(&path, self.filter_ignored) {
            Ok(tree) => {
                self.file_tree = Some(tree);
                self.selected_tree_file = None;
                self.preview_path = None;
                self.preview_content = None;
                self.selected_files.clear();
                self.export_modes.clear();
                self.active_list_index = None;
                self.clear_preview_cache();
                self.clear_checked();
                self.output_path = None;
                self.tree_search.clear();
                self.recompute_search();

                let (tx, rx) = std::sync::mpsc::channel();
                match new_debouncer(Duration::from_millis(200), tx) {
                    Ok(mut debouncer) => {
                        if debouncer
                            .watcher()
                            .watch(&path, RecursiveMode::Recursive)
                            .is_ok()
                        {
                            self.watcher = Some(debouncer);
                            self.watcher_rx = Some(rx);
                        } else {
                            self.watcher = None;
                            self.watcher_rx = None;
                        }
                    }
                    Err(_) => {
                        self.watcher = None;
                        self.watcher_rx = None;
                    }
                }

                self.status = format!("Opened folder: {}", path.display());
            }
            Err(e) => {
                self.status = format!("Error opening folder: {}", e);
            }
        }
    }

    pub fn refresh_tree(&mut self) {
        if let Some(root) = self.project_root.clone() {
            self.refresh_git_status();
            if let Ok(tree) = build_tree(&root, self.filter_ignored) {
                self.file_tree = Some(tree);
                self.recompute_search();
            }

            let before_len = self.selected_files.len();
            self.selected_files.retain(|p| p.exists());

            if self.selected_files.len() < before_len {
                self.checked_export_files
                    .retain(|p| self.selected_files.contains(p));
                self.export_modes
                    .retain(|p, _| self.selected_files.contains(p));

                if let Some(idx) = self.active_list_index {
                    if idx >= self.selected_files.len() {
                        self.active_list_index = if self.selected_files.is_empty() {
                            None
                        } else {
                            Some(self.selected_files.len() - 1)
                        };
                    }
                }
            }

            if let Some(preview_path) = &self.preview_path {
                if !preview_path.exists() {
                    self.close_preview();
                }
            }
        }
    }

    pub fn refresh_git_status(&mut self) {
        self.git_statuses.clear();
        let Some(root) = &self.project_root else {
            return;
        };

        if let Ok(repo) = git2::Repository::discover(root) {
            let mut opts = git2::StatusOptions::new();
            opts.include_untracked(true).recurse_untracked_dirs(true);

            if let Ok(statuses) = repo.statuses(Some(&mut opts)) {
                let repo_workdir = repo.workdir().unwrap_or(root);
                for entry in statuses.iter() {
                    let status = entry.status();
                    if let Some(path_str) = entry.path() {
                        let p = repo_workdir.join(path_str);
                        let gs =
                            if status.intersects(git2::Status::WT_NEW | git2::Status::INDEX_NEW) {
                                Some(GitStatus::Untracked)
                            } else if status.intersects(
                                git2::Status::WT_MODIFIED | git2::Status::INDEX_MODIFIED,
                            ) {
                                Some(GitStatus::Modified)
                            } else if status
                                .intersects(git2::Status::WT_DELETED | git2::Status::INDEX_DELETED)
                            {
                                Some(GitStatus::Deleted)
                            } else {
                                None
                            };

                        if let Some(s) = gs {
                            self.git_statuses.insert(p, s);
                        }
                    }
                }
            }
        }
    }

    pub fn set_filter_ignored(&mut self, on: bool) {
        if self.filter_ignored == on {
            return;
        }
        self.filter_ignored = on;
        self.refresh_tree();
        self.clear_checked();
        self.status = if on {
            "Tree filter: hiding ignored + hidden files".to_string()
        } else {
            "Tree filter: showing all files".to_string()
        };
    }

    pub fn select_output_path_dialog(&mut self) -> Option<PathBuf> {
        let mut dialog = FileDialog::new()
            .set_file_name("project-files.md")
            .add_filter("Markdown (*.md)", &["md"]);

        if let Some(ref current) = self.output_path {
            if let Some(parent) = current.parent() {
                dialog = dialog.set_directory(parent);
            }
        } else if let Some(ref last_out) = self.config.last_output_path {
            if let Some(parent) = last_out.parent() {
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
