use crate::fs_tree::TreeNode;
use crate::state::AppState;
use eframe::egui;

pub fn render(ui: &mut egui::Ui, state: &mut AppState) {
    ui.heading("Project Files");
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
