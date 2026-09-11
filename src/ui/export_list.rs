use crate::state::AppState;
use eframe::egui;

pub fn render(ui: &mut egui::Ui, state: &mut AppState) {
    let available_width = ui.available_width();
    let is_narrow = available_width < 360.0;

    let checked_count = state.checked_export_files.len();
    let has_items = !state.selected_files.is_empty();
    let has_active_selection = state.active_list_index.is_some() && has_items;

    if is_narrow {
        ui.vertical(|ui| {
            ui.heading("Export List");
            ui.add_space(2.0);

            ui.horizontal_wrapped(|ui| {
                if ui
                    .add_enabled(has_active_selection, egui::Button::new("Remove Selected"))
                    .clicked()
                {
                    state.remove_selected_from_export();
                }
                if ui
                    .add_enabled(has_items, egui::Button::new("Clear All"))
                    .clicked()
                {
                    state.clear_all_export_files();
                }
            });

            ui.separator();
            ui.add_space(2.0);

            ui.horizontal_wrapped(|ui| {
                if ui
                    .add_enabled(
                        checked_count > 0,
                        egui::Button::new(format!("Remove Checked ({})", checked_count)),
                    )
                    .clicked()
                {
                    state.remove_checked_from_export();
                }
                if ui
                    .add_enabled(checked_count > 0, egui::Button::new("Clear Checks"))
                    .clicked()
                {
                    state.clear_export_checked();
                    state.status = "Cleared export checks".to_string();
                }
            });
        });
    } else {
        ui.horizontal(|ui| {
            ui.heading("Export List");
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui
                    .add_enabled(has_items, egui::Button::new("Clear All"))
                    .clicked()
                {
                    state.clear_all_export_files();
                }
                if ui
                    .add_enabled(has_active_selection, egui::Button::new("Remove Selected"))
                    .clicked()
                {
                    state.remove_selected_from_export();
                }
            });
        });

        ui.separator();
        ui.add_space(2.0);

        ui.horizontal(|ui| {
            if ui
                .add_enabled(
                    checked_count > 0,
                    egui::Button::new(format!("Remove Checked ({})", checked_count)),
                )
                .clicked()
            {
                state.remove_checked_from_export();
            }
            if ui
                .add_enabled(checked_count > 0, egui::Button::new("Clear Checks"))
                .clicked()
            {
                state.clear_export_checked();
                state.status = "Cleared export checks".to_string();
            }
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

                let files = state.selected_files.clone();

                for (i, path) in files.iter().enumerate() {
                    let label = crate::state::to_rel_path_string(state.project_root.as_ref(), path);
                    let is_selected = state.active_list_index == Some(i);

                    ui.horizontal(|ui| {
                        let mut chk = state.checked_export_files.contains(path);
                        if ui.checkbox(&mut chk, "").changed() {
                            state.set_export_checked(path.clone(), chk);
                        }

                        if state.git_statuses.get(path) == Some(&crate::state::GitStatus::Modified)
                        {
                            let mode = state
                                .export_modes
                                .get(path)
                                .copied()
                                .unwrap_or(crate::state::ExportMode::Full);
                            let (label_text, color) = match mode {
                                crate::state::ExportMode::Full => {
                                    ("[Full]", ui.visuals().text_color())
                                }
                                crate::state::ExportMode::Diff => {
                                    ("[Diff]", egui::Color32::from_rgb(220, 160, 40))
                                }
                            };

                            let response = ui
                                .add(
                                    egui::Label::new(
                                        egui::RichText::new(label_text).color(color).size(11.0),
                                    )
                                    .sense(egui::Sense::click()),
                                )
                                .on_hover_cursor(egui::CursorIcon::PointingHand);

                            if response.clicked() {
                                let new_mode = match mode {
                                    crate::state::ExportMode::Full => {
                                        crate::state::ExportMode::Diff
                                    }
                                    crate::state::ExportMode::Diff => {
                                        crate::state::ExportMode::Full
                                    }
                                };
                                state.export_modes.insert(path.clone(), new_mode);
                            }
                        }

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

                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.add_space(14.0);
                            if ui
                                .small_button("−")
                                .on_hover_text("Remove from export list")
                                .clicked()
                            {
                                to_remove_idx = Some(i);
                            }
                        });
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
