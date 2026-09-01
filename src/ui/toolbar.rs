use crate::state::AppState;
use eframe::egui;

pub fn render(
    ui: &mut egui::Ui,
    state: &mut AppState,
    is_compact: bool,
    show_left_panel: &mut bool,
    show_right_panel: &mut bool,
) {
    ui.horizontal(|ui| {
        if is_compact {
            if ui.toggle_value(show_left_panel, "📁 Tree").clicked() {
                if *show_left_panel {
                    *show_right_panel = false;
                }
            }
            ui.separator();
        }

        ui.menu_button("File", |ui| {
            if ui
                .add(egui::Button::new("Open Folder...").shortcut_text("Ctrl+O"))
                .clicked()
            {
                state.open_folder_dialog();
                ui.close();
            }
            if ui
                .add(egui::Button::new("Save Output As...").shortcut_text("Ctrl+S"))
                .clicked()
            {
                state.select_output_path_dialog();
                ui.close();
            }
        });

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if is_compact {
                if ui.toggle_value(show_right_panel, "📝 List").clicked() {
                    if *show_right_panel {
                        *show_left_panel = false;
                    }
                }
                ui.separator();
            }

            let can_export = state.project_root.is_some() && !state.selected_files.is_empty();

            ui.menu_button("Output", |ui| {
                if ui
                    .add_enabled(
                        can_export,
                        egui::Button::new("Generate").shortcut_text("Ctrl+G"),
                    )
                    .clicked()
                {
                    state.generate_markdown();
                    ui.close();
                }
                if ui
                    .add_enabled(
                        can_export,
                        egui::Button::new("Append").shortcut_text("Ctrl+Shift+G"),
                    )
                    .clicked()
                {
                    state.append_markdown();
                    ui.close();
                }
            });
        });
    });
}
