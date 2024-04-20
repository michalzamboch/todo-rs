#![allow(dead_code)]

use backend::todo_dto::*;

#[derive(Debug)]
pub struct TodoCache {
    pub items: Vec<TodoDTO>,
    pub current: Option<TodoDTO>,
    pub new_title: String,
}

pub fn create_todo_cache() -> Box<TodoCache> {
    let handler = TodoCache {
        items: vec![],
        current: None,
        new_title: "".to_owned(),
    };

    Box::new(handler)
}
