use crate::state::AppState;
use eframe::egui;

pub fn render(ui: &mut egui::Ui, state: &mut AppState) {
    ui.horizontal(|ui| {
        if ui.button("Open Folder").clicked() {
            state.open_folder_dialog();
        }
        ui.separator();

        if ui.button("Save Output As...").clicked() {
            state.select_output_path_dialog();
        }

        let can_export = state.project_root.is_some() && !state.selected_files.is_empty();

        if ui
            .add_enabled(can_export, egui::Button::new("Generate"))
            .clicked()
        {
            state.generate_markdown();
        }

        if ui
            .add_enabled(can_export, egui::Button::new("Append"))
            .clicked()
        {
            state.append_markdown();
        }

        ui.separator();

        let add_enabled = state.selected_tree_file.is_some();
        if ui
            .add_enabled(add_enabled, egui::Button::new("Add Selected File"))
            .clicked()
        {
            if let Some(path) = state.selected_tree_file.clone() {
                state.add_file_to_export(path);
            }
        }

        let remove_enabled = state.active_list_index.is_some() && !state.selected_files.is_empty();
        if ui
            .add_enabled(remove_enabled, egui::Button::new("Remove Selected"))
            .clicked()
        {
            state.remove_selected_from_export();
        }

        let clear_enabled = !state.selected_files.is_empty();
        if ui
            .add_enabled(clear_enabled, egui::Button::new("Clear All"))
            .clicked()
        {
            state.selected_files.clear();
            state.active_list_index = None;
            state.status = "Cleared all selected files".to_string();
        }

        ui.separator();
        ui.label("CodeToMd");
    });
}
