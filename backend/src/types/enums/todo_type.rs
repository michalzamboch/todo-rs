#![allow(dead_code)]

use serde::*;
use std::fmt;

#[derive(Default, Debug, Copy, PartialEq, Eq, Clone, Hash, Serialize, Deserialize)]
pub enum TodoType {
    #[default]
    Generic,
    Archive,
    Trash,
}

impl fmt::Display for TodoType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TodoType::Generic => write!(f, "Generic"),
            TodoType::Archive => write!(f, "Archive"),
            TodoType::Trash => write!(f, "Trash"),
        }
    }
}
