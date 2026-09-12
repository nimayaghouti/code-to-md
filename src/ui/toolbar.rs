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
        if ui.toggle_value(show_left_panel, "📁 Tree").clicked() {
            if is_compact && *show_left_panel {
                *show_right_panel = false;
            }
        }
        ui.separator();

        let has_active_dir = state.project_root.is_some();
        let can_export = has_active_dir && !state.selected_files.is_empty();

        ui.menu_button("File", |ui| {
            if ui
                .add(egui::Button::new("Open Folder...").shortcut_text("Ctrl+O"))
                .clicked()
            {
                state.open_folder_dialog();
                ui.close();
            }

            if has_active_dir {
                let check_mark = if state.filter_ignored { "✔" } else { "" };
                if ui
                    .add(egui::Button::new("Hide ignored").shortcut_text(check_mark))
                    .on_hover_text("Hide hidden (dot) files and entries matched by .gitignore")
                    .clicked()
                {
                    state.set_filter_ignored(!state.filter_ignored);
                    ui.close();
                }

                if ui
                    .add(egui::Button::new("🔄 Refresh"))
                    .on_hover_text("Manually refresh the file tree")
                    .clicked()
                {
                    state.refresh_tree();
                    ui.close();
                }
            }

            ui.separator();

            if ui
                .add_enabled(
                    can_export,
                    egui::Button::new("Save").shortcut_text("Ctrl+S"),
                )
                .clicked()
            {
                state.save_markdown();
                ui.close();
            }

            if ui
                .add_enabled(
                    can_export,
                    egui::Button::new("Save As...").shortcut_text("Ctrl+Shift+S"),
                )
                .clicked()
            {
                state.save_as_markdown();
                ui.close();
            }

            if ui
                .add_enabled(
                    can_export,
                    egui::Button::new("Append").shortcut_text("Ctrl+Shift+A"),
                )
                .clicked()
            {
                state.append_markdown();
                ui.close();
            }
        });

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if ui.toggle_value(show_right_panel, "📝 List").clicked() {
                if is_compact && *show_right_panel {
                    *show_left_panel = false;
                }
            }
            ui.separator();
        });
    });
}
