use eframe::egui;
use eframe::egui::text::{LayoutJob, TextFormat};
use std::path::{Path, PathBuf};
use std::sync::{Arc, mpsc};
use syntect::easy::HighlightLines;
use syntect::parsing::{SyntaxReference, SyntaxSet};
use syntect::util::LinesWithEndings;

pub struct HighlightResult {
    pub generation: u64,
    pub path: PathBuf,
    pub size: usize,
    pub layout_job: LayoutJob,
}

fn find_syntax<'a>(syntax_set: &'a SyntaxSet, path: &Path) -> &'a SyntaxReference {
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_ascii_lowercase();

    let syntax_ext = match ext.as_str() {
        "js" | "mjs" | "cjs" | "jsx" => "js",
        "ts" | "mts" | "cts" | "tsx" => "js",

        "css" | "scss" | "sass" => "css",
        "vue" => "html",

        "json" | "jsonc" => "json",
        "yaml" | "yml" => "yaml",

        "html" | "htm" => "html",
        "svg" | "xml" => "xml",

        "md" | "markdown" => "md",

        "sh" | "bash" | "zsh" => "sh",

        "c" | "h" => "c",
        "cpp" | "cc" | "cxx" | "hpp" => "cpp",

        _ => ext.as_str(),
    };

    syntax_set
        .find_syntax_by_extension(syntax_ext)
        .unwrap_or_else(|| syntax_set.find_syntax_plain_text())
}

pub fn build_highlighted_layout(
    text: &str,
    path: &Path,
    syntax_set: &SyntaxSet,
    theme: &syntect::highlighting::Theme,
    font_id: &egui::FontId,
) -> LayoutJob {
    let mut layout_job = LayoutJob::default();
    let syntax = find_syntax(syntax_set, path);
    let mut highlighter = HighlightLines::new(syntax, theme);

    for line in LinesWithEndings::from(text) {
        let ranges = highlighter
            .highlight_line(line, syntax_set)
            .unwrap_or_default();

        for (style, component) in ranges {
            layout_job.append(
                component,
                0.0,
                TextFormat {
                    font_id: font_id.clone(),
                    color: egui::Color32::from_rgb(
                        style.foreground.r,
                        style.foreground.g,
                        style.foreground.b,
                    ),
                    ..Default::default()
                },
            );
        }
    }

    layout_job.wrap.max_width = f32::INFINITY;
    layout_job
}

pub fn spawn_highlight(
    text: String,
    path: PathBuf,
    generation: u64,
    syntax_set: Arc<SyntaxSet>,
    theme: Arc<syntect::highlighting::Theme>,
    font_id: egui::FontId,
) -> mpsc::Receiver<HighlightResult> {
    let (sender, receiver) = mpsc::channel();

    std::thread::spawn(move || {
        let size = text.len();

        let layout_job = build_highlighted_layout(&text, &path, &syntax_set, &theme, &font_id);

        let _ = sender.send(HighlightResult {
            generation,
            path,
            size,
            layout_job,
        });
    });

    receiver
}
