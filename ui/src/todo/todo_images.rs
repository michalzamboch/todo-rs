use eframe::egui::{self, *};

#[derive(Debug)]
pub struct TodoImages {
    pub trash_can: Image<'static>,
    pub sync: Image<'static>,
    pub close: Image<'static>,
}

pub fn create_todo_images() -> Box<TodoImages> {
    let trash_can = egui::Image::new(egui::include_image!("../../assets/images/bin.png"));
    let sync = egui::Image::new(egui::include_image!("../../assets/images/sync.png"));
    let close = egui::Image::new(egui::include_image!("../../assets/images/close.png"));

    let images = TodoImages {
        trash_can,
        sync,
        close,
    };

    Box::new(images)
}
