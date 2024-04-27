use std::error::Error;
use std::fmt;

use ::serde::*;
use chrono::*;

#[derive(Debug, Default, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct TodoDTO {
    id: u32,
    folder_id: Option<u32>,
    pub title: String,
    pub description: String,
    pub completed: bool,
    pub stared: bool,
    pub creation_time: DateTime<Local>,
    pub to_be_done_time: Option<DateTime<Local>>,
}

impl TodoDTO {
    pub fn new(id: u32, title: &str) -> Self {
        TodoDTO {
            id,
            title: title.to_owned(),
            creation_time: Local::now(),
            ..Default::default()
        }
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn folder_id(&self) -> Option<u32> {
        self.folder_id
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

        Ok(())
    }
}

impl fmt::Display for TodoDTO {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let result = format!(
            "ID: {} Folder ID: {} title: {}\n description: {} completed: {} stared: {}\n created: {} to be done: {}",
            self.id,
            self.folder_id.unwrap_or_default(),
            self.title,
            self.description,
            self.completed,
            self.stared,
            self.creation_time.to_string(),
            self.to_be_done_time.unwrap_or_default().to_string(),
        );
        write!(f, "{}", result)
    }
}
