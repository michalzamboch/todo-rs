#![allow(dead_code)]

use ::serde::*;
use types::enums::todo_type::*;
use uuid::*;

#[derive(Debug, Default, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct TodoDTO {
    id: u32,
    uuid: Uuid,
    title: String,
    completed: bool,
    todo_type: TodoType,
}

impl TodoDTO {
    pub fn new(id: u32, title: String) -> Self {
        TodoDTO {
            id,
            title,
            uuid: Uuid::new_v4(),
            ..Default::default()
        }
    }
}
