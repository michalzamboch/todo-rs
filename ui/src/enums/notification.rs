#![allow(dead_code)]

#[derive(Debug, Default, Clone, Hash, PartialEq, Eq)]
pub(crate) enum Notification {
    #[default]
    None,
    Info(String),
    Warning(String),
    Error(String),
}
