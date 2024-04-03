#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;

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
            ui.heading("Todo List");

            ui.vertical_centered(|ui| {
                let max_size = ui.max_rect();

                let mut child_ui = ui.child_ui(max_size, egui::Layout::top_down(egui::Align::Min));

                for todo in self.todos.iter_mut() {
                    child_ui.horizontal(|ui| {
                        ui.checkbox(&mut todo.checked, todo.title.as_str());
                    });
                }
            });
        });
    }
}
