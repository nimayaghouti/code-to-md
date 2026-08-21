use crate::preview::PreviewContent;
use crate::state::AppState;
use eframe::egui;

pub fn render(ui: &mut egui::Ui, state: &mut AppState) {
    ui.heading("Preview");
    ui.separator();
    if let Some(preview) = &state.preview_content {
        ui.label(format!("Path: {}", preview.path.display()));
        ui.label(format!("Size: {} bytes", preview.size));
        ui.separator();
        egui::ScrollArea::vertical()
            .max_height(ui.available_height())
            .show(ui, |ui| match &preview.content {
                PreviewContent::Text(text) => {
                    let mut text_ref = text.as_str();
                    ui.add(
                        egui::TextEdit::multiline(&mut text_ref)
                            .desired_width(f32::INFINITY)
                            .font(egui::TextStyle::Monospace),
                    );
                }
                PreviewContent::TooLarge => {
                    ui.label("File is too large to preview.");
                }
                PreviewContent::Binary => {
                    ui.label("Binary or unsupported text file.");
                }
                PreviewContent::Error(e) => {
                    ui.label(format!("Error: {}", e));
                }
            });
    } else if let Some(path) = &state.preview_path {
        ui.label(format!("Path: {}", path.display()));
        ui.label("Loading preview...");
    } else {
        ui.label("Select a file to preview");
    }
}
