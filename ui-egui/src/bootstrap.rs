#![allow(dead_code)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui::{self, *};

pub fn run() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };

    eframe::run_native(
        "My egui App",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Box::<MyApp>::default()
        }),
    )
}

struct MyApp {
    todos: Vec<ToDoItem>,
}

struct ToDoItem {
    checked: bool,
    title: String,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            todos: vec![
                ToDoItem {
                    checked: false,
                    title: "Learn Rust".to_owned(),
                },
                ToDoItem {
                    checked: false,
                    title: "Learn egui".to_owned(),
                },
                ToDoItem {
                    checked: false,
                    title: "Learn egui_extras".to_owned(),
                },
            ],
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("Todo List");
            });

            ui.vertical(|ui| {
                for item in self.todos.iter() {
                    ui.add_space(5.);

                    ui.horizontal(|ui| {
                        ui.with_layout(Layout::left_to_right(Align::LEFT), |ui| {
                            let check_btn = Button::new("❌").frame(false);
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
                }
            });
        });
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
