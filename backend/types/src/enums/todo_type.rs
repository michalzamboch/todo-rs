#![allow(dead_code)]

use serde::*;

#[derive(Default, Debug, Copy, PartialEq, Eq, Clone, Hash, Serialize, Deserialize)]
pub enum TodoType {
    #[default]
    Generic,
    Archive,
    Trash,
}
