use crate::state::{AppConfig, AppState};
use crate::ui;
use eframe::egui;
use std::path::PathBuf;

pub struct App {
    pub state: AppState,
    pub first_frame: bool,
    pub show_left_panel: bool,
    pub show_right_panel: bool,
    pub is_compact_mode: Option<bool>,
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>, cli_folder: Option<PathBuf>) -> Self {
        ui::fonts::setup(&cc.egui_ctx);

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
            }
        }

        Self {
            state,
            first_frame: true,
            show_left_panel: true,
            show_right_panel: true,
            is_compact_mode: None,
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

        ui::events::handle(ui, &mut self.state);

        if let Some(rx) = &self.state.watcher_rx {
            let mut changed = false;
            while let Ok(_) = rx.try_recv() {
                changed = true;
            }
            if changed {
                self.state.refresh_tree();
                ui.ctx().request_repaint();
            }
        }

        let screen_width = ui.max_rect().width();
        let is_compact = screen_width < 900.0;

        if Some(is_compact) != self.is_compact_mode {
            self.is_compact_mode = Some(is_compact);
            if is_compact {
                self.show_left_panel = false;
                self.show_right_panel = false;
            } else {
                self.show_left_panel = true;
                self.show_right_panel = true;
            }
        }

        let panel_frame = egui::Frame::central_panel(ui.style())
            .inner_margin(8.0)
            .shadow(egui::Shadow::NONE);

        egui::Panel::top("toolbar")
            .frame(panel_frame)
            .show(ui, |ui| {
                ui::toolbar::render(
                    ui,
                    &mut self.state,
                    is_compact,
                    &mut self.show_left_panel,
                    &mut self.show_right_panel,
                );
            });

        egui::Panel::bottom("status_bar")
            .frame(panel_frame)
            .show(ui, |ui| {
                ui::status_bar::render(ui, &self.state);
            });

        let central_rect = ui.available_rect_before_wrap();

        if is_compact {
            let drawer_frame = egui::Frame::window(ui.style())
                .inner_margin(8.0)
                .shadow(egui::Shadow::NONE);

            if self.show_left_panel {
                egui::Window::new("left_drawer")
                    .fixed_pos(central_rect.left_top())
                    .fixed_size([280.0, central_rect.height()])
                    .title_bar(false)
                    .resizable(false)
                    .frame(drawer_frame)
                    .show(ui.ctx(), |ui| {
                        ui::tree::render(ui, &mut self.state);
                    });
            }
            if self.show_right_panel {
                egui::Window::new("right_drawer")
                    .fixed_pos([central_rect.right() - 280.0, central_rect.top()])
                    .fixed_size([280.0, central_rect.height()])
                    .title_bar(false)
                    .resizable(false)
                    .frame(drawer_frame)
                    .show(ui.ctx(), |ui| {
                        ui::export_list::render(ui, &mut self.state);
                    });
            }
        } else {
            if self.show_left_panel {
                egui::Panel::left("file_tree")
                    .resizable(true)
                    .default_size(280.0)
                    .frame(panel_frame)
                    .show(ui, |ui| {
                        ui::tree::render(ui, &mut self.state);
                    });
            }
            if self.show_right_panel {
                egui::Panel::right("export_list")
                    .resizable(true)
                    .default_size(280.0)
                    .frame(panel_frame)
                    .show(ui, |ui| {
                        ui::export_list::render(ui, &mut self.state);
                    });
            }
        }

        egui::CentralPanel::default()
            .frame(panel_frame)
            .show(ui, |ui| {
                ui::preview::render(ui, &mut self.state);
            });
    }
}
