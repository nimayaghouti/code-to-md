use crate::state::AppState;
use eframe::egui;

pub fn render(ui: &mut egui::Ui, state: &mut AppState) {
    let available_width = ui.available_width();

    let render_buttons = |ui: &mut egui::Ui, state: &mut AppState| {
        let clear_enabled = !state.selected_files.is_empty();
        if ui
            .add_enabled(clear_enabled, egui::Button::new("Clear All"))
            .clicked()
        {
            state.selected_files.clear();
            state.active_list_index = None;
            state.status = "Cleared all selected files".to_string();
        }

        let remove_enabled = state.active_list_index.is_some() && !state.selected_files.is_empty();
        if ui
            .add_enabled(remove_enabled, egui::Button::new("Remove"))
            .clicked()
        {
            state.remove_selected_from_export();
        }
    };

    if available_width < 260.0 {
        ui.vertical(|ui| {
            ui.heading("Export List");
            ui.add_space(4.0);

            let layout = egui::Layout::right_to_left(egui::Align::TOP).with_main_wrap(true);
            ui.with_layout(layout, |ui| {
                render_buttons(ui, state);
            });
        });
    } else {
        ui.horizontal(|ui| {
            ui.heading("Export List");
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                render_buttons(ui, state);
            });
        });
    }

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
