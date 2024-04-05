#![allow(dead_code)]

#[derive(Debug, Default, Clone)]
pub struct TodoDTO {
    id: u32,
    title: String,
    completed: bool,
}

impl TodoDTO {
    pub fn new(id: u32, title: String) -> Self {
        TodoDTO {
            id,
            title,
            completed: false,
        }
    }
}
