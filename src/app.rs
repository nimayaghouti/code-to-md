use crate::state::AppState;
use crate::ui;
use eframe::egui;
use std::path::PathBuf;

pub struct App {
    pub state: AppState,
}

impl App {
    pub fn new(cli_folder: Option<PathBuf>) -> Self {
        Self {
            state: AppState::new(cli_folder),
        }
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
                    self.state.open_folder(path.to_path_buf());
                    break;
                } else {
                    self.state.status =
                        format!("Error: Dropped item is not a directory: {}", path.display());
                }
            }
        }

        // Render UI Panels
        egui::Panel::top("toolbar").show(ui, |ui| {
            ui::toolbar::render(ui, &mut self.state);
        });

        egui::Panel::bottom("status_bar").show(ui, |ui| {
            ui::status_bar::render(ui, &self.state);
        });

        egui::Panel::left("file_tree")
            .resizable(true)
            .default_size(300.0)
            .show(ui, |ui| {
                ui::tree::render(ui, &mut self.state);
            });

        egui::Panel::right("export_list")
            .resizable(true)
            .default_size(300.0)
            .show(ui, |ui| {
                ui::export_list::render(ui, &mut self.state);
            });

        egui::CentralPanel::default().show(ui, |ui| {
            ui::preview::render(ui, &mut self.state);
        });
    }
}
