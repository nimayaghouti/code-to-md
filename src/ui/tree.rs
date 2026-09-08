use crate::fs_tree::{TreeNode, collect_dirs, collect_files};
use crate::state::AppState;
use eframe::egui;
use eframe::egui::collapsing_header::CollapsingState;
use std::collections::HashSet;
use std::path::{Path, PathBuf};

fn node_id(path: &Path) -> egui::Id {
    egui::Id::new("tree_node").with(path)
}

pub fn render(ui: &mut egui::Ui, state: &mut AppState) {
    let available_width = ui.available_width();
    let is_narrow = available_width < 360.0;

    let add_enabled = state.selected_tree_file.is_some();

    if is_narrow {
        ui.vertical(|ui| {
            ui.heading("Project Files");
            ui.add_space(2.0);
            ui.horizontal_wrapped(|ui| {
                if ui
                    .add_enabled(add_enabled, egui::Button::new("Add Selected"))
                    .clicked()
                {
                    if let Some(path) = state.selected_tree_file.clone() {
                        state.add_file_to_export(path);
                    }
                }
            });
        });

        ui.separator();
        ui.add_space(2.0);
    } else {
        ui.horizontal(|ui| {
            ui.heading("Project Files");
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui
                    .add_enabled(add_enabled, egui::Button::new("Add Selected"))
                    .clicked()
                {
                    if let Some(path) = state.selected_tree_file.clone() {
                        state.add_file_to_export(path);
                    }
                }
            });
        });

        ui.separator();
        ui.add_space(2.0);
    }

    ui.horizontal(|ui| {
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if !state.tree_search.is_empty() {
                ui.label(format!("{}", state.search_match_count));
            }

            let resp = ui.add(
                egui::TextEdit::singleline(&mut state.tree_search)
                    .id_source("tree_search_input")
                    .desired_width(ui.available_width())
                    .font(egui::FontId::proportional(15.0))
                    .min_size(egui::vec2(0.0, 28.0))
                    .margin(egui::Margin::symmetric(8, 6))
                    .hint_text("Search files…"),
            );

            if resp.changed() {
                state.recompute_search();
            }
        });
    });

    ui.add_space(2.0);

    let checked_count = state.checked_files.len();

    if is_narrow {
        ui.horizontal_wrapped(|ui| {
            if ui
                .add_enabled(
                    checked_count > 0,
                    egui::Button::new(format!("Add Checked ({})", checked_count)),
                )
                .clicked()
            {
                state.add_checked_to_export();
            }
            if ui
                .add_enabled(checked_count > 0, egui::Button::new("Clear Checks"))
                .clicked()
            {
                state.clear_checked();
                state.status = "Cleared all checks".to_string();
            }
        });
    } else {
        ui.horizontal(|ui| {
            if ui
                .add_enabled(
                    checked_count > 0,
                    egui::Button::new(format!("Add Checked ({})", checked_count)),
                )
                .clicked()
            {
                state.add_checked_to_export();
            }
            if ui
                .add_enabled(checked_count > 0, egui::Button::new("Clear Checks"))
                .clicked()
            {
                state.clear_checked();
                state.status = "Cleared all checks".to_string();
            }
        });
    }

    ui.separator();

    let filter = state.search_matches.take();
    let tree = state.file_tree.take();

    egui::ScrollArea::vertical()
        .auto_shrink([false, false])
        .show(ui, |ui| {
            if let Some(tree) = &tree {
                render_tree_node(ui, state, tree, tree, true, filter.as_ref());
            } else {
                ui.label("No folder opened");
            }
        });

    state.file_tree = tree;
    state.search_matches = filter;
}

fn collapse_all(ctx: &egui::Context, root: &TreeNode) {
    let mut dirs = Vec::new();
    collect_dirs(root, &mut dirs);
    for p in dirs {
        let mut cs = CollapsingState::load_with_default_open(ctx, node_id(&p), false);
        if p == root.path {
            cs.set_open(true);
        } else {
            cs.set_open(false);
        }
        cs.store(ctx);
    }
}

#[allow(clippy::too_many_arguments)]
fn render_tree_node(
    ui: &mut egui::Ui,
    state: &mut AppState,
    node: &TreeNode,
    root: &TreeNode,
    is_root: bool,
    filter: Option<&HashSet<PathBuf>>,
) {
    if let Some(set) = filter {
        if !set.contains(&node.path) {
            return;
        }
    }

    if node.is_dir {
        let mut cs =
            CollapsingState::load_with_default_open(ui.ctx(), node_id(&node.path), is_root);
        if filter.is_some() {
            cs.set_open(true);
        }
        let open = cs.is_open();

        let mut toggle_clicked = false;
        let header = ui.horizontal(|ui| {
            let mut chk = state.checked_dirs.contains(&node.path);
            if ui.checkbox(&mut chk, "").changed() {
                let mut files = Vec::new();
                collect_files(node, &mut files);
                state.set_checked(files, chk);
                state.recompute_checked_dirs(root);
            }

            let tri = ui.selectable_label(false, if open { "▼" } else { "▶" });
            let name = ui.selectable_label(false, egui::RichText::new(&node.name).strong());
            if tri.clicked() || name.clicked() {
                toggle_clicked = true;
            }

            if is_root {
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.add_space(14.0);

                    let collapse_btn = egui::Button::new(
                        egui::RichText::new("▲")
                            .size(12.0)
                            .strong()
                            .color(ui.visuals().widgets.noninteractive.fg_stroke.color),
                    )
                    .min_size(egui::vec2(18.0, 18.0));

                    if ui.add(collapse_btn).on_hover_text("Collapse All").clicked() {
                        collapse_all(ui.ctx(), root);
                    }
                });
            }
        });

        if toggle_clicked {
            cs.set_open(!open);
            cs.store(ui.ctx());
        }

        cs.show_body_indented(&header.response, ui, |ui| {
            for child in &node.children {
                render_tree_node(ui, state, child, root, false, filter);
            }
        });
    } else {
        ui.horizontal(|ui| {
            let mut chk = state.checked_files.contains(&node.path);
            if ui.checkbox(&mut chk, "").changed() {
                state.set_checked(vec![node.path.clone()], chk);
                state.recompute_checked_dirs(root);
            }

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

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.add_space(14.0);
                if ui
                    .small_button("+")
                    .on_hover_text("Add to export list")
                    .clicked()
                {
                    state.add_file_to_export(node.path.clone());
                }
            });
        });
    }
}
