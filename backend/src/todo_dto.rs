use std::error::Error;
use std::fmt;

use ::serde::*;

use crate::types::enums::todo_type::*;

#[derive(Debug, Default, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct TodoDTO {
    id: u32,
    pub title: String,
    pub completed: bool,
    pub todo_type: TodoType,
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

    pub fn equal_by_id(&self, other: &TodoDTO) -> bool {
        self.id() == other.id()
    }

    pub fn update_from_equal(&mut self, other: TodoDTO) -> Result<(), Box<dyn Error>> {
        if self.id() != other.id() {
            return Err("IDs are not equal.".into());
        }

        self.id = other.id();
        self.title = other.title.to_owned();
        self.completed = other.completed;
        self.todo_type = other.todo_type;

        Ok(())
    }
}

impl fmt::Display for TodoDTO {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let result = format!(
            "ID: {} title: {} completed: {} type: {}",
            self.id, self.title, self.completed, self.todo_type
        );
        write!(f, "{}", result)
    }
}
