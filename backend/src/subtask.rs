use std::error::Error;
use std::fmt;

use ::serde::*;

#[derive(Debug, Default, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct Subtask {
    parent_id: u32,
    id: u32,
    pub title: String,
    pub completed: bool,
}

impl Subtask {
    pub fn new(parent_id: u32, id: u32, title: &str) -> Self {
        Self {
            parent_id,
            id,
            title: title.to_owned(),
            ..Default::default()
        }
    }

    pub fn parent_id(&self) -> u32 {
        self.parent_id
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn equal_by_id(&self, other: &Self) -> bool {
        self.id() == other.id()
    }

    pub fn update_from_equal(&mut self, other: Self) -> Result<(), Box<dyn Error>> {
        if self.id() != other.id() {
            return Err("IDs are not equal.".into());
        }

        self.parent_id = other.parent_id();
        self.id = other.id();
        self.title = other.title.to_owned();
        self.completed = other.completed;

        Ok(())
    }
}

impl fmt::Display for Subtask {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let result = format!(
            "ID: {} title: {} completed: {}",
            self.id, self.title, self.completed
        );
        write!(f, "{}", result)
    }
}
