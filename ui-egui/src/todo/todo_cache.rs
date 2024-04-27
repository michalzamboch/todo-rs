#![allow(dead_code)]

use backend::todo_dto::*;

#[derive(Debug, Default)]
pub struct TodoCache {
    pub undone: Vec<TodoDTO>,
    pub done: Vec<TodoDTO>,
    pub current: TodoDTO,
    pub current_selected: bool,
    pub new_title: String,
}

pub fn create_todo_cache() -> Box<TodoCache> {
    let handler = TodoCache {
        undone: vec![],
        done: vec![],
        new_title: "".to_owned(),
        ..Default::default()
    };

    Box::new(handler)
}
