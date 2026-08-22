use crate::state::{AppConfig, AppState};
use crate::ui;
use eframe::egui;
use std::path::PathBuf;

pub struct App {
    pub state: AppState,
    pub first_frame: bool,
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>, cli_folder: Option<PathBuf>) -> Self {
        let mut state = AppState::new(cli_folder.clone());

        if let Some(storage) = cc.storage {
            if let Some(config) = eframe::get_value::<AppConfig>(storage, eframe::APP_KEY) {
                state.config = config;

                if cli_folder.is_none() {
                    if let Some(ref last_root) = state.config.last_project_root {
                        if last_root.exists() && last_root.is_dir() {
                            state.open_folder(last_root.clone());
                        }
                    }
                }

                if let Some(ref last_out) = state.config.last_output_path {
                    state.output_path = Some(last_out.clone());
                }
            }
        }

        Self {
            state,
            first_frame: true,
        }
    }
}

impl eframe::App for App {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        self.state.config.last_project_root = self.state.project_root.clone();
        self.state.config.last_output_path = self.state.output_path.clone();
        eframe::set_value(storage, eframe::APP_KEY, &self.state.config);
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        if self.first_frame {
            ui.ctx()
                .send_viewport_cmd(egui::ViewportCommand::Visible(true));
            self.first_frame = false;
        }

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
