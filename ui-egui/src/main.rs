mod bootstrap;
mod constants;
mod todo_cache;
mod todo_handler;
mod todo_pipeline;

use crate::constants::*;
use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    run()
}

fn run() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size(DEFAULT_WINDOW_SIZE)
            .with_min_inner_size(DEFAULT_WINDOW_SIZE),
        ..Default::default()
    };

    eframe::run_native(
        "Todo-rs",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);

            bootstrap::create_filled_view()
        }),
    )
}
