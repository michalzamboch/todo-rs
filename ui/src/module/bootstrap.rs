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

    ui.run()
}
