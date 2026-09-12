use crate::preview::PreviewContent;
use crate::state::AppState;
use eframe::egui;

pub fn render(ui: &mut egui::Ui, state: &mut AppState) {
    state.poll_highlight_result();

    let min_width_required = 220.0;

    let draw_buttons = |ui: &mut egui::Ui, state: &mut AppState| {
        if state.preview_path.is_some() {
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button(" ✖ ").clicked() {
                    state.close_preview();
                }

                ui.separator();

                let mut is_diff = state.preview_mode == crate::state::PreviewMode::Diff;
                if ui.toggle_value(&mut is_diff, "Git Diff").clicked() {
                    state.preview_mode = if is_diff {
                        crate::state::PreviewMode::Diff
                    } else {
                        crate::state::PreviewMode::Source
                    };
                    if let Some(p) = state.preview_path.clone() {
                        state.load_preview(p);
                    }
                }
            });
        }
    };

    if ui.available_width() >= min_width_required {
        ui.horizontal(|ui| {
            ui.heading("Preview");
            draw_buttons(ui, state);
        });
    } else {
        ui.vertical(|ui| {
            ui.add(
                egui::Label::new(egui::RichText::new("Preview").heading())
                    .wrap_mode(egui::TextWrapMode::Extend),
            );
            ui.horizontal(|ui| {
                draw_buttons(ui, state);
            });
        });
    }

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
                    let mut generated_galley = None;

                    ui.horizontal(|ui| {
                        let fallback_lines;

                        let mut cache_key = preview_path.clone();
                        if state.preview_mode == crate::state::PreviewMode::Diff {
                            cache_key.set_extension("diff");
                        }

                        let line_numbers = if let Some(cached) = state.highlight_cache.get(&cache_key) {
                            &cached.line_numbers
                        } else {
                            let line_count = text.lines().count().max(1);
                            fallback_lines = (1..=line_count)
                                .map(|n| n.to_string())
                                .collect::<Vec<_>>()
                                .join("\n");
                            &fallback_lines
                        };

                        ui.add(egui::Label::new(
                            egui::RichText::new(line_numbers)
                                .font(font_id.clone())
                                .color(egui::Color32::from_gray(120)),
                        ));

                        ui.separator();

                        let mut text_ref = text.as_str();

                        ui.add(
                            egui::TextEdit::multiline(&mut text_ref)
                                .font(font_id.clone())
                                .desired_width(f32::INFINITY)
                                .interactive(true)
                                .layouter(
                                    &mut |ui: &egui::Ui, _text: &dyn egui::TextBuffer, _wrap_width: f32| {
                                        if let Some(cached) = state.highlight_cache.get(&cache_key) {
                                            if let Some(galley) = &cached.galley {
                                                return galley.clone();
                                            }

                                            let mut layout_job = cached.layout_job.clone();
                                            layout_job.wrap.max_width = f32::INFINITY;

                                            let galley = ui.fonts_mut(|f| f.layout_job(layout_job));
                                            generated_galley = Some(galley.clone());
                                            galley
                                        } else {
                                            let mut layout_job = egui::text::LayoutJob::default();
                                            layout_job.append(
                                                text,
                                                0.0,
                                                egui::text::TextFormat {
                                                    font_id: font_id.clone(),
                                                    color: ui.visuals().text_color(),
                                                    ..Default::default()
                                                },
                                            );
                                            layout_job.wrap.max_width = f32::INFINITY;

                                            ui.fonts_mut(|f| f.layout_job(layout_job))
                                        }
                                    },
                                ),
                        );
                    });

                    if let Some(galley) = generated_galley {
                        let mut cache_key = preview_path.clone();
                        if state.preview_mode == crate::state::PreviewMode::Diff {
                            cache_key.set_extension("diff");
                        }
                        if let Some(cached) = state.highlight_cache.get_mut(&cache_key) {
                            cached.galley = Some(galley);
                        }
                    }
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
        ui.add(egui::Label::new("Select a file to preview").wrap_mode(egui::TextWrapMode::Extend));
    }
}
