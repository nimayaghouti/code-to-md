#![windows_subsystem = "windows"]

use code_to_md::app::App;
use eframe::NativeOptions;
use std::env;
use std::path::PathBuf;
use std::sync::Arc;

fn load_icon() -> eframe::egui::IconData {
    let icon_bytes = include_bytes!("../assets/icon.ico");
    let image = image::load_from_memory(icon_bytes)
        .expect("Failed to load icon")
        .into_rgba8();

    let (width, height) = image.dimensions();
    eframe::egui::IconData {
        rgba: image.into_raw(),
        width,
        height,
    }
}

fn main() -> anyhow::Result<()> {
    let options = NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_app_id("code_to_md_app")
            .with_icon(Arc::new(load_icon()))
            .with_min_inner_size([450.0, 600.0])
            .with_visible(false),
        ..Default::default()
    };

    let cli_folder = env::args().nth(1).map(PathBuf::from);

    eframe::run_native(
        "CodeToMd",
        options,
        Box::new(move |cc| Ok(Box::new(App::new(cc, cli_folder)))),
    )?;
    Ok(())
}
