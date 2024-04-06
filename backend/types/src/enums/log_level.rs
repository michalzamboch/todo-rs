#![allow(dead_code)]

use serde::*;

#[derive(Default, Debug, PartialEq, Eq, Clone, Copy, Hash, Serialize, Deserialize)]
pub enum LogLevel {
    #[default]
    Info,
    Warning,
    Error,
}
