#![windows_subsystem = "windows"]

mod app;
mod fs_tree;
mod markdown;
mod preview;
mod language;

use eframe::NativeOptions;
use std::env;
use std::path::PathBuf;

fn main() -> anyhow::Result<()> {
    let options = NativeOptions::default();

    // Check for CLI argument (folder path)
    let cli_folder = env::args().nth(1).map(PathBuf::from);

    eframe::run_native(
        "CodeToMd",
        options,
        Box::new(move |_cc| Ok(Box::new(app::App::new(cli_folder)))),
    )?;
    Ok(())
}
