#![allow(dead_code)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use crate::{todo_cache::*, todo_handler::*, todo_pipeline::PipelineCommand::*};

use backend::model_handler::*;
use eframe::egui::{self, *};

pub fn create_filled_view() -> Box<AppView> {
    let model = create_new_handler();
    let todo_handler = create_todo_handler(model.todos());
    let todo_cache = create_todo_cache();

    let app_view = AppView {
        model,
        todo_handler,
        todo_cache,
    };

    Box::new(app_view)
}

#[derive(Debug)]
pub struct AppView {
    model: Box<ModelHandler>,
    todo_handler: Box<TodoViewHandler>,
    todo_cache: Box<TodoCache>,
}

impl eframe::App for AppView {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.process_pipeline();
        self.create_header(ctx);
        self.create_central_layout(ctx);
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

    fn create_central_layout(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.todo_main_view(ui);
        });
    }

    fn todo_main_view(&mut self, ui: &mut Ui) {
        ui.vertical(|ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.vertical(|ui| {
                    self.list_todos(ui);
                    ui.add_space(20.);
                });
            });
        });
    }

    fn list_todos(&mut self, ui: &mut Ui) {
        for item in self.todo_cache.items.iter_mut() {
            ui.add_space(5.);

            ui.horizontal(|ui| {
                ui.with_layout(Layout::left_to_right(Align::LEFT), |ui| {
                    let check_btn = Checkbox::without_text(&mut item.completed);
                    let check_btn_respose = ui.add(check_btn);
                    if check_btn_respose.clicked() {
                        self.todo_handler.push_command(Delete(item.clone()));
                        println!("Clicked check box: {}", item.title);
                    }

                    let title = Button::new(item.title.clone()).wrap(true).frame(false);
                    let title_response = ui.add(title);
                    if title_response.clicked() {
                        self.todo_cache.current = Some(item.clone());
                        println!("Clicked label: {}", item);
                    }
                });

                ui.with_layout(Layout::right_to_left(Align::RIGHT), |ui| {
                    let label = Label::new("1.1.2024".to_owned());
                    ui.add(label);
                });
            });

            ui.separator();
        }
    }

    fn add_todo_creation_bar(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            let width = ui.available_width();
            let input_line = TextEdit::singleline(&mut self.todo_cache.new_title)
                .desired_width(width)
                .hint_text("New todo");
            let response = ui.add(input_line);

            if response.lost_focus() && ui.input(|i| i.key_pressed(Key::Enter)) {
                let title = self.todo_cache.new_title.clone();
                self.todo_handler
                    .push_command(CreateUsingTitle(title.clone()));

                self.todo_cache.new_title.clear();
                println!("Created new todo with name: {}", title);
            }
        });
    }

    fn create_footer(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::bottom("Footer").show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.add_space(5.);
                self.add_todo_creation_bar(ui);
                ui.add_space(2.);
            });
        });
    }

    fn process_pipeline(&mut self) {
        let update = self.todo_handler.execute_pipeline();
        if update {
            self.todo_cache.items = self.model.todos().get_all();
        }
    }
}
