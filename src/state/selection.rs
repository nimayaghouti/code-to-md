use super::{AppState, ExportMode, to_rel_path_string};
use crate::fs_tree::TreeNode;
use std::collections::HashSet;
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
        self.selected_files.push(path.clone());
        self.export_modes.insert(path, ExportMode::Full);
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

        self.checked_export_files.remove(&removed);
        self.export_modes.remove(&removed);

        let rel = to_rel_path_string(self.project_root.as_ref(), &removed);
        self.status = format!("Removed file: {}", rel);

        if self.selected_files.is_empty() {
            self.active_list_index = None;
            self.preview_path = None;
            self.preview_content = None;
            self.clear_preview_cache();
            self.clear_export_checked();
            return;
        }

        let new_idx = if idx >= self.selected_files.len() {
            self.selected_files.len() - 1
        } else {
            idx
        };
        self.select_export_file(new_idx);
    }

    pub fn clear_all_export_files(&mut self) {
        self.selected_files.clear();
        self.checked_export_files.clear();
        self.export_modes.clear();
        self.active_list_index = None;
        self.clear_preview_cache();
        self.status = "Cleared all selected files".to_string();
    }

    pub fn set_checked(&mut self, paths: Vec<PathBuf>, checked: bool) {
        for p in paths {
            if checked {
                self.checked_files.insert(p);
            } else {
                self.checked_files.remove(&p);
            }
        }
    }

    pub fn recompute_checked_dirs(&mut self, tree: &TreeNode) {
        fn walk(
            node: &TreeNode,
            checked: &HashSet<PathBuf>,
            dirs: &mut HashSet<PathBuf>,
        ) -> (usize, usize) {
            if !node.is_dir {
                let c = usize::from(checked.contains(&node.path));
                return (1, c);
            }
            let (mut total, mut hit) = (0usize, 0usize);
            for c in &node.children {
                let (t, k) = walk(c, checked, dirs);
                total += t;
                hit += k;
            }
            if total > 0 && hit == total {
                dirs.insert(node.path.clone());
            }
            (total, hit)
        }
        let mut dirs = HashSet::new();
        walk(tree, &self.checked_files, &mut dirs);
        self.checked_dirs = dirs;
    }

    pub fn add_checked_to_export(&mut self) {
        let mut paths: Vec<PathBuf> = self.checked_files.iter().cloned().collect();
        paths.sort();
        let count = paths.len();
        for p in paths {
            self.add_file_to_export(p);
        }
        self.clear_checked();
        self.status = format!("Added {} checked file(s) to export list", count);
    }

    pub fn clear_checked(&mut self) {
        self.checked_files.clear();
        self.checked_dirs.clear();
    }

    pub fn recompute_search(&mut self) {
        let q = self.tree_search.trim().to_lowercase();
        if q.is_empty() {
            self.search_matches = None;
            self.search_match_count = 0;
            return;
        }

        fn walk(node: &TreeNode, q: &str, set: &mut HashSet<PathBuf>, count: &mut usize) -> bool {
            if !node.is_dir {
                let m = node.name.to_lowercase().contains(q);
                if m {
                    set.insert(node.path.clone());
                    *count += 1;
                }
                return m;
            }
            let mut any = false;
            for c in &node.children {
                any |= walk(c, q, set, count);
            }
            if any {
                set.insert(node.path.clone());
            }
            any
        }

        let mut set = HashSet::new();
        let mut count = 0usize;
        if let Some(tree) = &self.file_tree {
            walk(tree, &q, &mut set, &mut count);
        }
        self.search_matches = Some(set);
        self.search_match_count = count;
    }

    pub fn set_export_checked(&mut self, path: PathBuf, checked: bool) {
        if checked {
            self.checked_export_files.insert(path);
        } else {
            self.checked_export_files.remove(&path);
        }
    }

    pub fn remove_checked_from_export(&mut self) {
        let count = self.checked_export_files.len();
        self.selected_files
            .retain(|p| !self.checked_export_files.contains(p));
        self.export_modes
            .retain(|p, _| self.selected_files.contains(p));
        self.checked_export_files.clear();
        self.active_list_index = None;
        self.clear_preview_cache();
        self.status = format!("Removed {} checked file(s) from export list", count);
    }

    pub fn clear_export_checked(&mut self) {
        self.checked_export_files.clear();
    }
}
