use crate::state::AppState;
use eframe::egui;

pub fn render(ui: &mut egui::Ui, state: &AppState) {
    let out_str = state
        .output_path
        .as_ref()
        .map(|p| p.display().to_string())
        .unwrap_or_else(|| "Not set".to_string());

    let available_width = ui.available_width();

    let status_galley = egui::WidgetText::from(format!("Status: {}", state.status)).into_galley(
        ui,
        None,
        f32::INFINITY,
        egui::FontSelection::Default,
    );
    let output_galley = egui::WidgetText::from(format!("Output File: {}", out_str)).into_galley(
        ui,
        None,
        f32::INFINITY,
        egui::FontSelection::Default,
    );

    let min_width_needed = status_galley.size().x + output_galley.size().x + 40.0;

    if available_width >= min_width_needed {
        ui.horizontal(|ui| {
            ui.label("Status:");
            ui.label(&state.status);

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.add(egui::Label::new(&out_str).wrap());
                ui.label("Output File:");
            });
        });
    } else {
        ui.horizontal_wrapped(|ui| {
            ui.label("Status:");
            ui.label(&state.status);

            ui.add_space(16.0);

            ui.label("Output File:");
            ui.add(egui::Label::new(out_str).wrap());
        });
    }
}
