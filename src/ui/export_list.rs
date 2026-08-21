use crate::state::AppState;
use eframe::egui;

pub fn render(ui: &mut egui::Ui, state: &mut AppState) {
    ui.heading("Export List");
    ui.separator();

    egui::ScrollArea::both()
        .auto_shrink([false, false])
        .show(ui, |ui| {
            if state.selected_files.is_empty() {
                ui.label("No files added to Markdown yet.");
            } else {
                let mut clicked_idx = None;
                let mut to_remove_idx = None;

                for (i, path) in state.selected_files.iter().enumerate() {
                    let label = crate::state::to_rel_path_string(state.project_root.as_ref(), path);
                    let is_selected = state.active_list_index == Some(i);
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
                    state.remove_file_by_index(idx);
                } else if let Some(idx) = clicked_idx {
                    state.select_export_file(idx);
                }
            }
        });
}
