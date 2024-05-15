#![allow(dead_code)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use crate::{constants::*, todo::todo_ui::*};

use backend::model_handler::*;
use eframe::egui::{self, *};

pub fn run() -> Result<(), eframe::Error> {
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

            create_filled_view()
        }),
    )
}

fn create_filled_view() -> Box<AppView> {
    let model = create_new_handler();
    let todo_view = create_filled_todo_view(model.todos().clone());

    let app_view = AppView { model, todo_view };

    Box::new(app_view)
}

#[derive(Debug)]
struct AppView {
    model: Box<ModelHandler>,
    todo_view: Box<TodoView>,
}

impl eframe::App for AppView {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        set_font(ctx);

        if ctx.input(|i| i.key_pressed(Key::Escape)) {
            ctx.send_viewport_cmd(ViewportCommand::Close);
        }
        self.todo_view.update(ctx);
    }

    fn save(&mut self, _storage: &mut dyn eframe::Storage) {}

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        println!("Exit");
    }

    fn auto_save_interval(&self) -> std::time::Duration {
        std::time::Duration::from_secs(30)
    }

    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        egui::Color32::from_rgba_unmultiplied(12, 12, 12, 180).to_normalized_gamma_f32()
    }

    fn persist_egui_memory(&self) -> bool {
        true
    }
}

fn set_font(ctx: &Context) {
    let mut fonts = FontDefinitions::default();
    fonts.font_data.insert(
        "droid_sans_propo".to_owned(),
        FontData::from_static(include_bytes!("../assets/fonts/DroidSansMono/DroidSansMNerdFontPropo-Regular.otf")),
    );
    fonts.font_data.insert(
        "droid_sans_mono".to_owned(),
        FontData::from_static(include_bytes!("../assets/fonts/DroidSansMono/DroidSansMNerdFontMono-Regular.otf")),
    );

    fonts
        .families
        .get_mut(&FontFamily::Proportional)
        .unwrap()
        .insert(0, "droid_sans_propo".to_owned());

    fonts
        .families
        .get_mut(&FontFamily::Monospace)
        .unwrap()
        .push("droid_sans_mono".to_owned());

    ctx.set_fonts(fonts);
}
