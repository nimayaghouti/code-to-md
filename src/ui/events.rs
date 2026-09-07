use crate::state::AppState;
use eframe::egui;

pub fn handle(ui: &mut egui::Ui, state: &mut AppState) {
    if ui.input_mut(|i| {
        i.consume_shortcut(&egui::KeyboardShortcut::new(
            egui::Modifiers::CTRL,
            egui::Key::O,
        ))
    }) {
        state.open_folder_dialog();
    }

    let can_export = state.project_root.is_some() && !state.selected_files.is_empty();

    if ui.input_mut(|i| {
        i.consume_shortcut(&egui::KeyboardShortcut::new(
            egui::Modifiers::CTRL | egui::Modifiers::SHIFT,
            egui::Key::S,
        ))
    }) {
        if can_export {
            state.save_as_markdown();
        }
    }

    if ui.input_mut(|i| {
        i.consume_shortcut(&egui::KeyboardShortcut::new(
            egui::Modifiers::CTRL,
            egui::Key::S,
        ))
    }) {
        if can_export {
            state.save_markdown();
        }
    }

    if ui.input_mut(|i| {
        i.consume_shortcut(&egui::KeyboardShortcut::new(
            egui::Modifiers::CTRL | egui::Modifiers::SHIFT,
            egui::Key::A,
        ))
    }) {
        if can_export {
            state.append_markdown();
        }
    }

    let dropped_files = ui.ctx().input(|i| i.raw.dropped_files.clone());
    if !dropped_files.is_empty() {
        for dropped_file in dropped_files {
            let path = dropped_file.path();
            if path.as_os_str().is_empty() {
                continue;
            }

            if path.is_dir() {
                state.open_folder(path.to_path_buf());
                break;
            } else {
                state.status =
                    format!("Error: Dropped item is not a directory: {}", path.display());
            }
        }
    }
}
