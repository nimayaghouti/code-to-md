pub mod export;
pub mod io;
pub mod preview;
pub mod selection;

use crate::fs_tree::TreeNode;
use crate::preview::FilePreview;
use eframe::egui;
use notify::RecommendedWatcher;
use notify_debouncer_mini::{DebouncedEvent, Debouncer};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::sync::{Arc, mpsc};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum GitStatus {
    Modified,
    Untracked,
    Deleted,
    Added,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ExportMode {
    Full,
    Diff,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PreviewMode {
    Source,
    Diff,
}

#[derive(Default, Deserialize, Serialize)]
#[serde(default)]
pub struct AppConfig {
    pub last_project_root: Option<PathBuf>,
    pub last_output_path: Option<PathBuf>,
    pub was_maximized: bool,
}

pub struct CachedHighlight {
    pub layout_job: egui::text::LayoutJob,
    pub line_numbers: String,
    pub galley: Option<std::sync::Arc<egui::Galley>>,
    pub size: usize,
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
    pub syntax_set: Arc<syntect::parsing::SyntaxSet>,
    pub theme: Arc<syntect::highlighting::Theme>,
    pub highlight_cache: HashMap<PathBuf, CachedHighlight>,
    pub highlight_cache_size: usize,
    pub highlight_receiver: Option<mpsc::Receiver<crate::highlight::HighlightResult>>,
    pub highlight_generation: u64,
    pub filter_ignored: bool,
    pub tree_search: String,
    pub search_matches: Option<HashSet<PathBuf>>,
    pub search_match_count: usize,
    pub checked_files: HashSet<PathBuf>,
    pub checked_dirs: HashSet<PathBuf>,
    pub checked_export_files: HashSet<PathBuf>,
    pub watcher: Option<Debouncer<RecommendedWatcher>>,
    pub watcher_rx: Option<mpsc::Receiver<Result<Vec<DebouncedEvent>, notify::Error>>>,
    pub git_statuses: HashMap<PathBuf, GitStatus>,
    pub export_modes: HashMap<PathBuf, ExportMode>,
    pub preview_mode: PreviewMode,
}

impl AppState {
    pub fn new(cli_folder: Option<PathBuf>) -> Self {
        let syntax_set = Arc::new(syntect::parsing::SyntaxSet::load_defaults_newlines());
        let theme_set = syntect::highlighting::ThemeSet::load_defaults();
        let theme = Arc::new(
            theme_set
                .themes
                .get("base16-ocean.dark")
                .expect("base16-ocean.dark theme should exist")
                .clone(),
        );
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
            syntax_set,
            theme,
            highlight_cache: HashMap::new(),
            highlight_cache_size: 0,
            highlight_receiver: None,
            highlight_generation: 0,
            filter_ignored: true,
            tree_search: String::new(),
            search_matches: None,
            search_match_count: 0,
            checked_files: HashSet::new(),
            checked_dirs: HashSet::new(),
            checked_export_files: HashSet::new(),
            watcher: None,
            watcher_rx: None,
            git_statuses: HashMap::new(),
            export_modes: HashMap::new(),
            preview_mode: PreviewMode::Source,
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
