#![allow(dead_code, unused_imports)]

use crate::include::*;
use backend::todo_dto::*;
use slint::*;

pub fn todos_to_modelrc(todos: &[TodoDTO]) -> ModelRc<ViewToDoItem> {
    let tmp = todos_to_ui_elements(todos);
    ModelRc::from(tmp.as_slice())
}

pub fn todos_to_ui_elements(todos: &[TodoDTO]) -> Vec<ViewToDoItem> {
    todos.iter().map(todo_to_viewtodo).collect()
}

pub fn todo_to_viewtodo(todo: &TodoDTO) -> ViewToDoItem {
    ViewToDoItem {
        id: todo.id() as i32,
        title: todo.title().into(),
        checked: todo.completed(),
    }
}
