#![allow(dead_code)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::time::Duration;

use crate::{constants::*, enums::notification::*, todo::todo_ui::*};

use backend::model_handler::*;
use eframe::egui::{self, *};
use egui_notify::*;

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

            create_filled_view(cc)
        }),
    )
}

fn create_filled_view(cc: &eframe::CreationContext<'_>) -> Box<AppView> {
    let model = create_new_handler();
    let todo_view = create_filled_todo_view(model.todos().clone());
    let toasts = Box::<Toasts>::default();

    let app_view = AppView {
        model,
        todo_view,
        toasts,
    };

    set_font(&cc.egui_ctx);
    set_text_sizes(&cc.egui_ctx);
    set_empty_toasts(&cc.egui_ctx);
    Box::new(app_view)
}

struct AppView {
    model: Box<ModelHandler>,
    todo_view: Box<TodoView>,
    toasts: Box<Toasts>,
}

impl AppView {
    fn set_all_toasts(&mut self, ctx: &Context) {
        let mut clear = false;

        ctx.data(|r| {
            let toasts: Option<Vec<Notification>> = r.get_temp(TOASTS.into());

            for n in &toasts.unwrap_or_default() {
                clear = true;
                let duration = Some(Duration::from_secs(5));

                match n {
                    Notification::None => (),
                    Notification::Info(msg) => {
                        self.toasts.info(msg).set_duration(duration);
                    }
                    Notification::Warning(msg) => {
                        self.toasts.warning(msg).set_duration(duration);
                    }
                    Notification::Error(msg) => {
                        self.toasts.error(msg).set_duration(duration);
                    }
                }
            }
        });

        if clear {
            ctx.data_mut(|w| {
                let empty: Vec<Notification> = vec![];
                w.get_temp_mut_or(TOASTS.into(), empty).clear();
            });
        }
    }
}

impl eframe::App for AppView {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if ctx.input(|i| i.key_pressed(Key::Escape)) {
            ctx.send_viewport_cmd(ViewportCommand::Close);
        }

        self.todo_view.update(ctx);
        self.set_all_toasts(ctx);
        self.toasts.show(ctx);
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
        FontData::from_static(include_bytes!(
            "../assets/fonts/DroidSansMono/DroidSansMNerdFontPropo-Regular.otf"
        )),
    );
    fonts.font_data.insert(
        "droid_sans_mono".to_owned(),
        FontData::from_static(include_bytes!(
            "../assets/fonts/DroidSansMono/DroidSansMNerdFontMono-Regular.otf"
        )),
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

fn set_text_sizes(ctx: &egui::Context) {
    use FontFamily::Proportional;
    use TextStyle::*;

    let mut style = (*ctx.style()).clone();
    style.text_styles = [
        (Heading, FontId::new(25.0, Proportional)),
        (Body, FontId::new(15.0, Proportional)),
        (Monospace, FontId::new(15.0, Proportional)),
        (Button, FontId::new(15.0, Proportional)),
        (Small, FontId::new(9.0, Proportional)),
    ]
    .into();
    ctx.set_style(style);
}

fn set_empty_toasts(ctx: &Context) {
    ctx.data_mut(|w| {
        let empty_toasts: Vec<Notification> = vec![];
        w.insert_temp(TOASTS.into(), empty_toasts);
    });
}
