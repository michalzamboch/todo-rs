#![allow(dead_code)]

use backend::{todo_dto::*, todo_filter::*, types::traits::dao::*};
use eframe::egui::{self, *};

use super::{
    todo_cache::*,
    todo_handler::*,
    todo_pipeline::PipelineCommand::{self, *},
};

#[derive(Debug)]
pub struct TodoView {
    dao: DaoRef<TodoDTO>,
    handler: Box<TodoViewHandler>,
    cache: Box<TodoCache>,
}

pub fn create_filled_todo_view(dao: DaoRef<TodoDTO>) -> Box<TodoView> {
    let handler = create_todo_handler(dao.clone());
    let cache = create_todo_cache();

    let app_view = TodoView {
        dao,
        handler,
        cache,
    };

    Box::new(app_view)
}

impl TodoView {
    pub fn update(&mut self, ctx: &egui::Context) {
        self.process_pipeline();
        self.create_header(ctx);
        self.create_right_panel(ctx);
        self.create_central_layout(ctx);
        self.create_footer(ctx);
    }

    fn create_header(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("todo_header").show(ctx, |ui| {
            ui.add_space(5.);
            ui.horizontal(|ui| {
                let clear_icon = egui::include_image!("../../assets/images/bin.png");
                let clear_btn = Button::image_and_text(clear_icon, "CLear");
                let response = ui.add(clear_btn);

                if response.clicked() {
                    let remove: Box<[PipelineCommand]> =
                        self.cache.done.clone().into_iter().map(Delete).collect();

                    self.handler.push_commands(&remove);
                }

                let load_icon = egui::include_image!("../../assets/images/sync.png");
                let load_btn = Button::image_and_text(load_icon, "Load");
                let response = ui.add(load_btn);

                if response.clicked() {
                    self.handler.push_command(Load);
                    self.cache.current_selected = false;
                    self.cache.activate_search = false;
                    self.cache.search_term.clear();
                }

                ui.with_layout(Layout::right_to_left(egui::Align::RIGHT), |ui| {
                    self.create_search_bar(ui);
                });
            });

            ui.add_space(5.);
        });
    }

    fn create_search_bar(&mut self, ui: &mut Ui) {
        if self.cache.activate_search {
            let cancel_icon = egui::include_image!("../../assets/images/close.png");
            let cancel_btn = Button::image_and_text(cancel_icon, "Cancel");
            let response = ui.add(cancel_btn);

            if response.clicked() {
                self.cache.activate_search = false;
                self.cache.search_term.clear();
                self.handler.push_command(Load);
            }
        }

        let search_line = TextEdit::singleline(&mut self.cache.search_term).hint_text("Search");
        let response = ui.add(search_line);

        if response.lost_focus() && ui.input(|i| i.key_pressed(Key::Enter)) {
            self.cache.activate_search = true;
            self.handler.push_command(Load);
        }
    }

    fn create_right_panel(&mut self, ctx: &Context) {
        let show = self.cache.current_selected;
        if show {
            egui::SidePanel::right("todo_detail")
                .max_width(400.)
                .min_width(175.)
                .default_width(225.)
                .resizable(true)
                .show(ctx, |ui| {
                    ui.vertical(|ui| {
                        self.display_selected_todo(ui);
                    });
                });
        }
    }

    fn display_selected_todo(&mut self, ui: &mut Ui) {
        ui.add_space(5.);
        self.create_title_edit(ui);
        ui.add_space(5.);
        self.create_description_edit(ui);
    }

    fn create_time_created(&mut self, ui: &mut Ui) {
        let curent = &self.cache.current;
        ui.label("Created:");
        ui.label(&curent.creation_time_fmt());
    }

    fn create_title_edit(&mut self, ui: &mut Ui) {
        let curent = &mut self.cache.current;

        ui.label("Title");
        ui.with_layout(Layout::top_down_justified(Align::Max), |ui| {
            let title_edit = TextEdit::singleline(&mut curent.title);
            let response = ui.add(title_edit);

            if response.lost_focus() && ui.input(|i| i.key_pressed(Key::Enter)) {
                let copy = curent.clone();
                self.handler.push_command(Update(copy));
            }
        });
    }

    fn create_description_edit(&mut self, ui: &mut Ui) {
        let curent = &mut self.cache.current;

        ui.label("Description");
        let desc_edit =
            TextEdit::multiline(&mut curent.description).hint_text("Set description...");
        let response = ui.add_sized(ui.available_size(), desc_edit);

        if response.lost_focus() {
            let copy = curent.clone();
            self.handler.push_command(Update(copy));
        }
    }

    fn create_central_layout(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.todo_main_view(ui);
        });
    }

    fn todo_main_view(&mut self, ui: &mut Ui) {
        ui.vertical(|ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                self.list_undone_todos(ui);

                if !self.cache.done.is_empty() {
                    ui.collapsing("Done", |ui| {
                        self.list_done_todos(ui);
                    });
                }

                ui.add_space(30.);
            });
        });
    }

    fn list_undone_todos(&mut self, ui: &mut Ui) {
        for item in self.cache.undone.iter_mut() {
            ui.add_space(5.);

            ui.horizontal(|ui| {
                ui.with_layout(Layout::left_to_right(Align::LEFT), |ui| {
                    let check_btn = Checkbox::without_text(&mut item.completed);
                    let check_btn_respose = ui.add(check_btn);

                    if check_btn_respose.clicked() {
                        if item.equal_by_id(&self.cache.current) {
                            self.cache.current_selected = false;
                        }

                        self.handler.push_command(Update(item.clone()));
                        println!("Clicked check box: {}", item.title);
                    }
                });

                ui.with_layout(Layout::right_to_left(Align::BOTTOM), |ui| {
                    ui.add_space(10.);
                    ui.label("1.1.2024");

                    ui.with_layout(Layout::top_down_justified(egui::Align::LEFT), |ui| {
                        let title = SelectableLabel::new(false, &item.title);
                        let title_response = ui.add(title);

                        if title_response.clicked() {
                            self.cache.current = item.clone();
                            self.cache.current_selected = true;
                            println!("Clicked label: {}", item);
                        }
                    });
                });
            });

            ui.separator();
        }
    }

    fn list_done_todos(&mut self, ui: &mut Ui) {
        for item in self.cache.done.iter_mut() {
            ui.add_space(5.);

            ui.horizontal(|ui| {
                ui.with_layout(Layout::left_to_right(Align::LEFT), |ui| {
                    let check_btn = Checkbox::without_text(&mut item.completed);
                    let check_btn_respose = ui.add(check_btn);
                    if check_btn_respose.clicked() {
                        self.handler.push_command(Update(item.clone()));
                        println!("Clicked check box: {}", item.title);
                    }

                    ui.label(&item.title);

                    ui.with_layout(Layout::right_to_left(Align::RIGHT), |ui| {
                        ui.add_space(10.);

                        let remove_btn = Button::new("❌").frame(false);
                        let response = ui.add(remove_btn);
                        if response.clicked() {
                            self.handler.push_command(Delete(item.clone()));
                        }
                    });
                });
            });

            ui.separator();
        }
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

    fn add_todo_creation_bar(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            let width = ui.available_width();
            let input_line = TextEdit::singleline(&mut self.cache.new_title)
                .desired_width(width)
                .hint_text("New todo");
            let response = ui.add(input_line);

            if response.lost_focus() && ui.input(|i| i.key_pressed(Key::Enter)) {
                let title = self.cache.new_title.clone();
                self.handler.push_command(CreateUsingTitle(title.clone()));

                self.cache.new_title.clear();
            }
        });
    }

    fn process_pipeline(&mut self) {
        let update = self.handler.execute_pipeline();
        if update {
            self.data_update();
        }
    }

    fn data_update(&mut self) {
        let all_todos = self.dao.get_all();
        let divided = split_done_undone(&all_todos);
        self.cache.done = divided.0;

        if self.cache.activate_search {
            let search_term = &self.cache.search_term;
            let filtered = FilterTodosBy::new().title(search_term).filter(&divided.1);
            self.cache.undone = filtered;
        } else {
            self.cache.undone = divided.1;
        }
    }
}

fn menu_bar(ui: &mut Ui) {
    menu::bar(ui, |ui| {
        ui.menu_button("File", |ui| {
            if ui.button("Open").clicked() {
                // …
            }
        });
    });
}
