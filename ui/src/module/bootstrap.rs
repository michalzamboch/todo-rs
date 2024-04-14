#![allow(dead_code, unused_variables)]

use crate::include::*;
use slint::*;

use super::wrappers::*;
use backend::model_handler::*;

pub fn run() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    let model = create_new_handler();
    let tmp_todos = model.todos().get_all();
    let todos = todos_to_modelrc(&tmp_todos);
    ui.set_todo_items(todos);

    set_events(&ui);

    ui.run()
}

fn set_events(ui: &AppWindow) {
    let weak_ui = ui.as_weak();
    ui.on_item_checked(move |ui_item| {
        let rc_ui = weak_ui.unwrap();
    });
}
