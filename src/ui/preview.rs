use crate::preview::PreviewContent;
use crate::state::AppState;
use eframe::egui;

pub fn render(ui: &mut egui::Ui, state: &mut AppState) {
    state.poll_highlight_result();

    ui.horizontal(|ui| {
        ui.heading("Preview");

        if state.preview_path.is_some() {
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button(" ✖ ").clicked() {
                    state.close_preview();
                }
            });
        }
    });

    ui.separator();

    if let Some(preview) = &state.preview_content {
        let preview_path = preview.path.clone();

        ui.horizontal(|ui| {
            ui.label(format!("Path: {}", preview_path.display()));
            ui.separator();
            ui.label(format!("Size: {} bytes", preview.size));
        });

        ui.separator();

        egui::ScrollArea::both()
            .auto_shrink([false, false])
            .show(ui, |ui| match &preview.content {
                PreviewContent::Text(text) => {
                    let font_id = egui::FontId::new(14.0, egui::FontFamily::Monospace);

                    ui.horizontal(|ui| {
                        let line_count = text.lines().count().max(1);

                        let line_numbers = (1..=line_count)
                            .map(|n| n.to_string())
                            .collect::<Vec<_>>()
                            .join("\n");

                        ui.add(egui::Label::new(
                            egui::RichText::new(line_numbers)
                                .font(font_id.clone())
                                .color(egui::Color32::from_gray(120)),
                        ));

                        ui.separator();

                        let mut text_ref = text.as_str();

                        ui.add(
                            egui::TextEdit::multiline(
                                &mut text_ref,
                            )
                            .font(font_id.clone())
                            .desired_width(f32::INFINITY)
                            .interactive(true)
                            .layouter(
                                &mut |
                                    ui: &egui::Ui,
                                    _text: &dyn egui::TextBuffer,
                                    wrap_width: f32,
                                | {
                                    if let Some(cached) =
                                        state.highlight_cache.get(
                                            &preview_path,
                                        )
                                    {
                                        let mut layout_job =
                                            cached.layout_job.clone();

                                        layout_job.wrap.max_width =
                                            wrap_width;

                                        ui.fonts_mut(|f| {
                                            f.layout_job(layout_job)
                                        })
                                    } else {
                                        let mut layout_job =
                                            egui::text::LayoutJob::default();

                                        layout_job.append(
                                            text,
                                            0.0,
                                            egui::text::TextFormat {
                                                font_id: font_id.clone(),
                                                color: ui
                                                    .visuals()
                                                    .text_color(),
                                                ..Default::default()
                                            },
                                        );

                                        layout_job.wrap.max_width =
                                            wrap_width;

                                        ui.fonts_mut(|f| {
                                            f.layout_job(layout_job)
                                        })
                                    }
                                },
                            ),
                        );
                    });
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
