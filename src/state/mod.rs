pub mod export;
pub mod io;
pub mod selection;

use crate::fs_tree::TreeNode;
use crate::preview::FilePreview;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Default, Deserialize, Serialize)]
pub struct AppConfig {
    pub last_project_root: Option<PathBuf>,
    pub last_output_path: Option<PathBuf>,
}

pub struct AppState {
    pub config: AppConfig,
    pub status: String,
    pub project_root: Option<PathBuf>,
    pub file_tree: Option<TreeNode>,
    pub selected_tree_file: Option<PathBuf>,
    pub preview_path: Option<PathBuf>,
    pub preview_content: Option<FilePreview>,
    pub selected_files: Vec<PathBuf>,
    pub active_list_index: Option<usize>,
    pub output_path: Option<PathBuf>,
}

impl AppState {
    pub fn new(cli_folder: Option<PathBuf>) -> Self {
        let mut state = Self {
            config: AppConfig::default(),
            status: "Ready".to_string(),
            project_root: None,
            file_tree: None,
            selected_tree_file: None,
            preview_path: None,
            preview_content: None,
            selected_files: Vec::new(),
            active_list_index: None,
            output_path: None,
        };

        if let Some(folder) = cli_folder {
            if folder.is_dir() {
                state.open_folder(folder);
            } else {
                state.status = format!(
                    "Error: CLI argument is not a valid directory: {}",
                    folder.display()
                );
            }
        }
        state
    }
}

pub fn is_binary_bytes(bytes: &[u8]) -> bool {
    let check_len = bytes.len().min(1024);
    bytes[..check_len].contains(&0)
}

pub fn to_rel_path_string(root: Option<&PathBuf>, path: &Path) -> String {
    let rel = root.and_then(|r| path.strip_prefix(r).ok()).unwrap_or(path);
    let posix_path = rel.to_string_lossy().replace('\\', "/");
    format!("@/{}", posix_path)
}
