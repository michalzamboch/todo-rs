#![allow(dead_code)]

use backend::todo_dto::*;

#[derive(Debug, Default)]
pub struct TodoCache {
    pub undone: Vec<TodoDTO>,
    pub done: Vec<TodoDTO>,
    pub current: Option<TodoDTO>,
    pub new_title: String,
}

pub fn create_todo_cache() -> Box<TodoCache> {
    let handler = TodoCache {
        undone: vec![],
        done: vec![],
        current: None,
        new_title: "".to_owned(),
    };

    Box::new(handler)
}
