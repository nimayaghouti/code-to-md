use crate::state::AppState;
use eframe::egui;

pub fn render(ui: &mut egui::Ui, state: &AppState) {
    ui.horizontal(|ui| {
        ui.label("Status:");
        ui.label(&state.status);

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            let out_str = state
                .output_path
                .as_ref()
                .map(|p| p.display().to_string())
                .unwrap_or_else(|| "Not set".to_string());
            ui.label(out_str);
            ui.label("Output File:");
        });
    });
}
