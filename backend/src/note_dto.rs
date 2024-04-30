use std::error::Error;
use std::fmt;

use ::serde::*;

#[derive(Debug, Default, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct NoteDTO {
    id: u32,
    pub title: String,
    pub text: String,
}

impl NoteDTO {
    pub fn new(id: u32, title: &str) -> Self {
        Self {
            id,
            title: title.to_owned(),
            ..Default::default()
        }
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

        self.id = other.id();
        self.title = other.title.to_owned();

        Ok(())
    }
}

impl fmt::Display for NoteDTO {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let result = format!("ID: {} title: {} text: {}", self.id, self.title, self.text);
        write!(f, "{}", result)
    }
}
