use crate::fs_tree::{TreeNode, build_tree};
use crate::preview::{FilePreview, PreviewContent, load_preview};
use eframe::egui;
use rfd::FileDialog;
use std::path::{Path, PathBuf};

pub struct App {
    status: String,
    project_root: Option<PathBuf>,
    file_tree: Option<TreeNode>,
    selected_tree_file: Option<PathBuf>,
    preview_path: Option<PathBuf>,
    preview_content: Option<FilePreview>,
    selected_files: Vec<PathBuf>,
    active_list_index: Option<usize>,
    output_path: Option<PathBuf>,
}

impl App {
    pub fn new(cli_folder: Option<PathBuf>) -> Self {
        let mut app = Self {
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
                app.open_folder(folder);
            } else {
                app.status = format!(
                    "Error: CLI argument is not a valid directory: {}",
                    folder.display()
                );
            }
        }
        app
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new(None)
    }
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // Handle drag and drop events
        let dropped_files = ui.ctx().input(|i| i.raw.dropped_files.clone());
        if !dropped_files.is_empty() {
            for dropped_file in dropped_files {
                let path = dropped_file.path();
                if path.as_os_str().is_empty() {
                    continue;
                }

                if path.is_dir() {
                    self.open_folder(path.to_path_buf());
                    break;
                } else {
                    self.status =
                        format!("Error: Dropped item is not a directory: {}", path.display());
                }
            }
        }

        // Top toolbar
        egui::Panel::top("toolbar").show(ui, |ui| {
            ui.horizontal(|ui| {
                if ui.button("Open Folder").clicked() {
                    self.open_folder_dialog();
                }
                ui.separator();

                if ui.button("Save Output As...").clicked() {
                    self.select_output_path_dialog();
                }

                let can_export = self.project_root.is_some() && !self.selected_files.is_empty();

                if ui
                    .add_enabled(can_export, egui::Button::new("Generate"))
                    .clicked()
                {
                    self.generate_markdown();
                }

                if ui
                    .add_enabled(can_export, egui::Button::new("Append"))
                    .clicked()
                {
                    self.append_markdown();
                }

                ui.separator();

                let add_enabled = self.selected_tree_file.is_some();
                if ui
                    .add_enabled(add_enabled, egui::Button::new("Add Selected File"))
                    .clicked()
                {
                    if let Some(path) = self.selected_tree_file.clone() {
                        self.add_file_to_export(path);
                    }
                }

                let remove_enabled =
                    self.active_list_index.is_some() && !self.selected_files.is_empty();
                if ui
                    .add_enabled(remove_enabled, egui::Button::new("Remove Selected"))
                    .clicked()
                {
                    self.remove_selected_from_export();
                }

                let clear_enabled = !self.selected_files.is_empty();
                if ui
                    .add_enabled(clear_enabled, egui::Button::new("Clear All"))
                    .clicked()
                {
                    self.selected_files.clear();
                    self.active_list_index = None;
                    self.status = "Cleared all selected files".to_string();
                }

                ui.separator();
                ui.label("CodeToMd");
            });
        });

        // Bottom panel - Status bar & Output path
        egui::Panel::bottom("status_bar").show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label("Status:");
                ui.label(&self.status);

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let out_str = self
                        .output_path
                        .as_ref()
                        .map(|p| p.display().to_string())
                        .unwrap_or_else(|| "Not set".to_string());
                    ui.label(out_str);
                    ui.label("Output File:");
                });
            });
        });

        // Left panel - file tree
        egui::Panel::left("file_tree")
            .resizable(true)
            .default_size(300.0)
            .show(ui, |ui| {
                ui.heading("Project Files");
                ui.separator();

                egui::ScrollArea::both()
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        if let Some(tree) = self.file_tree.clone() {
                            self.render_tree_node(ui, &tree, true);
                        } else {
                            ui.label("No folder opened");
                        }
                    });
            });

        // Right panel - Export list
        egui::Panel::right("export_list")
            .resizable(true)
            .default_size(300.0)
            .show(ui, |ui| {
                ui.heading("Export List");
                ui.separator();

                egui::ScrollArea::both()
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        if self.selected_files.is_empty() {
                            ui.label("No files added to Markdown yet.");
                        } else {
                            let mut clicked_idx = None;
                            let mut to_remove_idx = None;

                            for (i, path) in self.selected_files.iter().enumerate() {
                                let label = to_rel_path_string(self.project_root.as_ref(), path);
                                let is_selected = self.active_list_index == Some(i);
                                let response = ui.selectable_label(is_selected, label);

                                if response.clicked() {
                                    clicked_idx = Some(i);
                                }

                                response.context_menu(|ui| {
                                    if ui.button("Remove from Markdown").clicked() {
                                        to_remove_idx = Some(i);
                                        ui.close();
                                    }
                                });
                            }

                            if let Some(idx) = to_remove_idx {
                                self.remove_file_by_index(idx);
                            } else if let Some(idx) = clicked_idx {
                                self.select_export_file(idx);
                            }
                        }
                    });
            });

        // Central panel - preview with scrolling
        egui::CentralPanel::default().show(ui, |ui| {
            ui.heading("Preview");
            ui.separator();
            if let Some(preview) = &self.preview_content {
                ui.label(format!("Path: {}", preview.path.display()));
                ui.label(format!("Size: {} bytes", preview.size));
                ui.separator();
                egui::ScrollArea::vertical()
                    .max_height(ui.available_height())
                    .show(ui, |ui| match &preview.content {
                        PreviewContent::Text(text) => {
                            let mut text_ref = text.as_str();
                            ui.add(
                                egui::TextEdit::multiline(&mut text_ref)
                                    .desired_width(f32::INFINITY)
                                    .font(egui::TextStyle::Monospace),
                            );
                        }
                        PreviewContent::TooLarge => {
                            ui.label("File is too large to preview.");
                        }
                        PreviewContent::Binary => {
                            ui.label("Binary or unsupported text file.");
                        }
                        PreviewContent::Error(e) => {
                            ui.label(format!("Error: {}", e));
                        }
                    });
            } else if let Some(path) = &self.preview_path {
                ui.label(format!("Path: {}", path.display()));
                ui.label("Loading preview...");
            } else {
                ui.label("Select a file to preview");
            }
        });
    }
}

impl App {
    fn open_folder_dialog(&mut self) {
        if let Some(path) = FileDialog::new().pick_folder() {
            self.open_folder(path);
        }
    }

    fn open_folder(&mut self, path: PathBuf) {
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

    fn select_output_path_dialog(&mut self) -> Option<PathBuf> {
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

    fn render_tree_node(&mut self, ui: &mut egui::Ui, node: &TreeNode, is_root: bool) {
        if node.is_dir {
            egui::CollapsingHeader::new(&node.name)
                .default_open(is_root)
                .show(ui, |ui| {
                    for child in &node.children {
                        self.render_tree_node(ui, child, false);
                    }
                });
        } else {
            let is_selected = self.selected_tree_file.as_ref() == Some(&node.path);
            let response = ui.selectable_label(is_selected, &node.name);

            if response.clicked() {
                self.select_tree_file(node.path.clone());
            }

            response.context_menu(|ui| {
                if ui.button("Add to Markdown").clicked() {
                    self.add_file_to_export(node.path.clone());
                    ui.close();
                }
            });
        }
    }

    fn select_tree_file(&mut self, path: PathBuf) {
        self.selected_tree_file = Some(path.clone());
        self.active_list_index = None;
        self.load_preview(path);
    }

    fn select_export_file(&mut self, idx: usize) {
        if idx < self.selected_files.len() {
            self.active_list_index = Some(idx);
            self.selected_tree_file = None;
            let path = self.selected_files[idx].clone();
            self.load_preview(path);
        }
    }

    fn add_file_to_export(&mut self, path: PathBuf) {
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

    fn remove_selected_from_export(&mut self) {
        if let Some(idx) = self.active_list_index {
            self.remove_file_by_index(idx);
        }
    }

    fn remove_file_by_index(&mut self, idx: usize) {
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

    fn load_preview(&mut self, path: PathBuf) {
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

    fn generate_markdown(&mut self) {
        let Some(ref root) = self.project_root.clone() else {
            return;
        };
        if self.selected_files.is_empty() {
            self.status = "No files selected for export".to_string();
            return;
        }

        let output_path = if let Some(ref path) = self.output_path {
            path.clone()
        } else if let Some(path) = self.select_output_path_dialog() {
            path
        } else {
            self.status = "Generate canceled: no output path chosen".to_string();
            return;
        };

        let mut output_text = String::new();
        let mut written_count = 0;
        let mut skipped_count = 0;

        for path in &self.selected_files {
            if !path.exists() || !path.is_file() {
                skipped_count += 1;
                continue;
            }

            match std::fs::read(path) {
                Ok(bytes) => {
                    if is_binary_bytes(&bytes) {
                        skipped_count += 1;
                        continue;
                    }
                    match String::from_utf8(bytes) {
                        Ok(content) => {
                            let entry =
                                crate::markdown::generate_markdown_entry(root, path, &content);
                            output_text.push_str(&entry);
                            written_count += 1;
                        }
                        Err(_) => {
                            skipped_count += 1;
                        }
                    }
                }
                Err(_) => {
                    skipped_count += 1;
                }
            }
        }

        match std::fs::write(&output_path, output_text) {
            Ok(_) => {
                self.status = format!(
                    "Generated {} file(s) into {}. Skipped {} file(s).",
                    written_count,
                    output_path.display(),
                    skipped_count
                );
            }
            Err(e) => {
                self.status = format!("Error writing output file: {}", e);
            }
        }
    }

    fn append_markdown(&mut self) {
        let Some(ref root) = self.project_root.clone() else {
            return;
        };
        if self.selected_files.is_empty() {
            self.status = "No files selected for export".to_string();
            return;
        }

        let output_path = if let Some(ref path) = self.output_path {
            path.clone()
        } else if let Some(path) = self.select_output_path_dialog() {
            path
        } else {
            self.status = "Append canceled: no output path chosen".to_string();
            return;
        };

        let mut output_text = String::new();
        let mut written_count = 0;
        let mut skipped_count = 0;

        for path in &self.selected_files {
            if !path.exists() || !path.is_file() {
                skipped_count += 1;
                continue;
            }

            match std::fs::read(path) {
                Ok(bytes) => {
                    if is_binary_bytes(&bytes) {
                        skipped_count += 1;
                        continue;
                    }
                    match String::from_utf8(bytes) {
                        Ok(content) => {
                            let entry =
                                crate::markdown::generate_markdown_entry(root, path, &content);
                            output_text.push_str(&entry);
                            written_count += 1;
                        }
                        Err(_) => {
                            skipped_count += 1;
                        }
                    }
                }
                Err(_) => {
                    skipped_count += 1;
                }
            }
        }

        let mut prefix = String::new();
        if output_path.exists() {
            if let Ok(existing) = std::fs::read_to_string(&output_path) {
                if !existing.is_empty() && !existing.ends_with('\n') {
                    prefix.push('\n');
                }
            }
        }

        let mut file = match std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&output_path)
        {
            Ok(f) => f,
            Err(e) => {
                self.status = format!("Error opening output file: {}", e);
                return;
            }
        };

        use std::io::Write;
        let mut to_write = prefix;
        to_write.push_str(&output_text);

        match file.write_all(to_write.as_bytes()) {
            Ok(_) => {
                self.status = format!(
                    "Appended {} file(s) to {}. Skipped {} file(s).",
                    written_count,
                    output_path.display(),
                    skipped_count
                );
            }
            Err(e) => {
                self.status = format!("Error appending to output file: {}", e);
            }
        }
    }
}

fn is_binary_bytes(bytes: &[u8]) -> bool {
    let check_len = bytes.len().min(1024);
    bytes[..check_len].contains(&0)
}

fn to_rel_path_string(root: Option<&PathBuf>, path: &Path) -> String {
    let rel = root.and_then(|r| path.strip_prefix(r).ok()).unwrap_or(path);
    let posix_path = rel.to_string_lossy().replace('\\', "/");
    format!("@/{}", posix_path)
}
