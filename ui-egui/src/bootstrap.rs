#![allow(dead_code)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use backend::model_handler::{create_new_handler, ModelHandler};
use eframe::egui::{self, *};

use crate::constants::*;

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
    let app_view = AppView{
        model: create_new_handler(),
        todos: create_view_todos(),
    };

    Box::new(app_view)
}

fn create_view_todos() -> Vec<ToDoItem> {
    let mut todos = Vec::new();

    todos.push(ToDoItem {
        checked: false,
        title: "Learn Rust".into(),
    });
    todos.push(ToDoItem {
        checked: false,
        title: "Learn Egui".into(),
    });
    todos.push(ToDoItem {
        checked: false,
        title: "Learn Slint".into(),
    });

    todos
}

#[derive(Debug)]
struct AppView {
    model: Box<ModelHandler>,
    todos: Vec<ToDoItem>,
}

#[derive(Debug, Default, Clone, Hash, PartialEq, Eq)]
struct ToDoItem {
    checked: bool,
    title: String,
}

impl AppView {
    fn create_header(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("Header").show(ctx, |ui| {
            ui.add_space(5.);
            ui.vertical_centered(|ui| {
                ui.heading("Todo List");
            });

            ui.add_space(5.);
        });
    }

    fn create_vertical_layout(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.vertical(|ui| {
                    for item in self.todos.iter_mut() {
                        ui.add_space(5.);

                        ui.horizontal(|ui| {
                            ui.with_layout(Layout::left_to_right(Align::LEFT), |ui| {
                                let check_btn = Checkbox::without_text(&mut item.checked);
                                let check_btn_respose = ui.add(check_btn);
                                if check_btn_respose.clicked() {
                                    println!("Clicked check box: {}", item.title);
                                }

                                let title_label =
                                    Button::new(item.title.clone()).wrap(true).frame(false);
                                let title_label_response = ui.add(title_label);
                                if title_label_response.clicked() {
                                    println!("Clicked label: {}", item.title);
                                }
                            });

                            ui.with_layout(Layout::right_to_left(Align::RIGHT), |ui| {
                                let label = Label::new("1.1.2024".to_owned());
                                ui.add(label);
                            });
                        });

                        ui.separator();
                    }
                });
            });
        });
    }

    fn create_footer(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::bottom("Footer").show(ctx, |ui| {
            ui.add_space(5.);
        });
    }
}

impl eframe::App for AppView {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.create_header(ctx);
        self.create_vertical_layout(ctx);
        self.create_footer(ctx);
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
