#![allow(dead_code)]

use ::serde::*;
use types::enums::todo_type::*;

#[derive(Debug, Default, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct TodoDTO {
    id: u32,
    title: String,
    completed: bool,
    todo_type: TodoType,
}

impl TodoDTO {
    pub fn new(id: u32, title: &str) -> Self {
        TodoDTO {
            id,
            title: title.to_owned(),
            ..Default::default()
        }
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn completed(&self) -> bool {
        self.completed
    }

    pub fn todo_type(&self) -> TodoType {
        self.todo_type
    }

    pub fn set_title(&mut self, title: &str) {
        self.title = title.to_owned();
    }

    pub fn set_completed(&mut self, completed: bool) {
        self.completed = completed;
    }

    pub fn set_todo_type(&mut self, todo_type: TodoType) {
        self.todo_type = todo_type;
    }

    pub fn equal_by_id(&self, other: &TodoDTO) -> bool {
        self.id() == other.id()
    }
}
