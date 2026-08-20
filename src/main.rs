#![windows_subsystem = "windows"]

use code_to_md::app::App;
use eframe::NativeOptions;
use std::env;
use std::path::PathBuf;

fn main() -> anyhow::Result<()> {
    let options = NativeOptions::default();
    let cli_folder = env::args().nth(1).map(PathBuf::from);

    eframe::run_native(
        "CodeToMd",
        options,
        Box::new(move |_cc| Ok(Box::new(App::new(cli_folder)))),
    )?;
    Ok(())
}
