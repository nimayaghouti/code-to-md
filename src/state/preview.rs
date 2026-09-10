use super::{AppState, CachedHighlight, PreviewMode};
use crate::highlight::{HighlightResult, spawn_highlight};
use crate::preview::{FilePreview, PreviewContent, load_preview};
use eframe::egui;
use std::path::PathBuf;

const MAX_HIGHLIGHT_CACHE_SIZE: usize = 8 * 1024 * 1024;

impl AppState {
    pub fn load_preview(&mut self, path: PathBuf) {
        self.highlight_generation = self.highlight_generation.wrapping_add(1);

        let generation = self.highlight_generation;

        self.highlight_receiver = None;
        self.preview_path = Some(path.clone());

        let mut cache_key = path.clone();
        if self.preview_mode == PreviewMode::Diff {
            cache_key.set_extension("diff");
        }

        let content_result = if self.preview_mode == PreviewMode::Diff {
            if let Some(diff) =
                crate::state::export::get_file_diff(self.project_root.as_ref().unwrap(), &path)
            {
                Ok(FilePreview {
                    path: path.clone(),
                    size: diff.len() as u64,
                    content: PreviewContent::Text(diff),
                })
            } else {
                Ok(FilePreview {
                    path: path.clone(),
                    size: 0,
                    content: PreviewContent::Text("No changes or file untracked.".to_string()),
                })
            }
        } else {
            load_preview(&path)
        };

        match content_result {
            Ok(preview) => {
                self.status = format!(
                    "Loaded {} bytes: {}",
                    preview.size,
                    self.preview_relative_path(&path),
                );

                if let PreviewContent::Text(text) = &preview.content {
                    if !self.is_highlight_cached(&cache_key, text.len()) {
                        let font_id = egui::FontId::new(14.0, egui::FontFamily::Monospace);

                        self.highlight_receiver = Some(spawn_highlight(
                            text.clone(),
                            cache_key.clone(),
                            generation,
                            self.syntax_set.clone(),
                            self.theme.clone(),
                            font_id,
                        ));
                    }
                }

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

    pub fn poll_highlight_result(&mut self) {
        let result = match self.highlight_receiver.as_ref() {
            Some(receiver) => match receiver.try_recv() {
                Ok(result) => Some(result),
                Err(std::sync::mpsc::TryRecvError::Empty) => None,
                Err(std::sync::mpsc::TryRecvError::Disconnected) => None,
            },
            None => None,
        };

        let Some(result) = result else {
            return;
        };

        self.highlight_receiver = None;

        if result.generation != self.highlight_generation {
            return;
        }

        let mut expected_path = self.preview_path.clone().unwrap_or_default();
        if self.preview_mode == PreviewMode::Diff {
            expected_path.set_extension("diff");
        }

        if result.path != expected_path {
            return;
        }

        self.cache_highlight(result);
    }

    fn is_highlight_cached(&self, path: &PathBuf, size: usize) -> bool {
        self.highlight_cache
            .get(path)
            .map(|cached| cached.size == size)
            .unwrap_or(false)
    }

    fn cache_highlight(&mut self, result: HighlightResult) {
        if let Some(previous) = self.highlight_cache.remove(&result.path) {
            self.highlight_cache_size = self.highlight_cache_size.saturating_sub(previous.size);
        }

        if self.highlight_cache_size + result.size > MAX_HIGHLIGHT_CACHE_SIZE {
            self.highlight_cache.clear();
            self.highlight_cache_size = 0;
        }

        self.highlight_cache.insert(
            result.path,
            CachedHighlight {
                layout_job: result.layout_job,
                line_numbers: result.line_numbers,
                galley: None,
                size: result.size,
            },
        );

        self.highlight_cache_size += result.size;
    }

    pub fn clear_preview_cache(&mut self) {
        self.highlight_cache.clear();
        self.highlight_cache_size = 0;
        self.highlight_receiver = None;
        self.highlight_generation = self.highlight_generation.wrapping_add(1);
    }

    fn preview_relative_path(&self, path: &PathBuf) -> String {
        self.project_root
            .as_ref()
            .and_then(|root| path.strip_prefix(root).ok())
            .unwrap_or(path)
            .display()
            .to_string()
    }

    pub fn close_preview(&mut self) {
        self.preview_path = None;
        self.preview_content = None;
        self.selected_tree_file = None;
        self.active_list_index = None;
        self.highlight_receiver = None;
        self.highlight_generation = self.highlight_generation.wrapping_add(1);
    }
}
