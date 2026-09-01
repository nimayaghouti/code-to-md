use crate::fs_tree::TreeNode;
use crate::state::AppState;
use eframe::egui;

pub fn render(ui: &mut egui::Ui, state: &mut AppState) {
    let available_width = ui.available_width();

    let render_buttons = |ui: &mut egui::Ui, state: &mut AppState| {
        let add_enabled = state.selected_tree_file.is_some();
        if ui
            .add_enabled(add_enabled, egui::Button::new("Add Selected"))
            .clicked()
        {
            if let Some(path) = state.selected_tree_file.clone() {
                state.add_file_to_export(path);
            }
        }
    };

    if available_width < 240.0 {
        ui.vertical(|ui| {
            ui.heading("Project Files");
            ui.add_space(4.0);
            ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                render_buttons(ui, state);
            });
        });
    } else {
        ui.horizontal(|ui| {
            ui.heading("Project Files");
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                render_buttons(ui, state);
            });
        });
    }

    ui.separator();

    egui::ScrollArea::both()
        .auto_shrink([false, false])
        .show(ui, |ui| {
            if let Some(tree) = state.file_tree.clone() {
                render_tree_node(ui, state, &tree, true);
            } else {
                ui.label("No folder opened");
            }
        });
}

fn render_tree_node(ui: &mut egui::Ui, state: &mut AppState, node: &TreeNode, is_root: bool) {
    if node.is_dir {
        egui::CollapsingHeader::new(&node.name)
            .default_open(is_root)
            .show(ui, |ui| {
                for child in &node.children {
                    render_tree_node(ui, state, child, false);
                }
            });
    } else {
        let is_selected = state.selected_tree_file.as_ref() == Some(&node.path);
        let response = ui.selectable_label(is_selected, &node.name);

        if response.clicked() {
            state.select_tree_file(node.path.clone());
        }

        response.context_menu(|ui| {
            if ui.button("Add to Markdown").clicked() {
                state.add_file_to_export(node.path.clone());
                ui.close();
            }
        });
    }
}
