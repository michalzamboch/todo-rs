#![allow(dead_code)]

use ::serde::*;
use std::fmt;

use crate::todo_dto::*;

#[derive(Debug, Default, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct FilterTodosBy {
    id_range: Option<(u32, u32)>,
    title: Option<String>,
    completed: Option<bool>,
}

impl FilterTodosBy {
    pub fn new() -> Self {
        FilterTodosBy {
            ..Default::default()
        }
    }

    pub fn filter(&self, todos: &[TodoDTO]) -> Vec<TodoDTO> {
        let mut result: Vec<TodoDTO> = vec![];

        if let Some(x) = self.id_range {
            let mut tmp: Vec<TodoDTO> = todos
                .iter()
                .filter(|&i| i.id() >= x.0 && i.id() <= x.1)
                .cloned()
                .collect();
            result.append(&mut tmp);
        }

        result
    }

    pub fn id_range(mut self, range: (u32, u32)) -> Self {
        if range.0 > range.1 {
            self.id_range = None;
        } else {
            self.id_range = Some(range);
        }
        self
    }

    pub fn title(mut self, title: &str) -> Self {
        self.title = Some(title.to_owned());
        self
    }

    pub fn completed(mut self, completed: bool) -> Self {
        self.completed = Some(completed);
        self
    }
}

impl fmt::Display for FilterTodosBy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let id_range = match self.id_range {
            Some(range) => format!("{} - {} ", range.0, range.1),
            None => "".to_owned(),
        };

        let title = match self.title.clone() {
            Some(s) => s,
            None => "".to_owned(),
        };

        let completed = match self.completed {
            Some(c) => c.to_string(),
            None => "".to_owned(),
        };

        let result = format!("{id_range}{title}{completed}");
        write!(f, "{}", result)
    }
}

pub fn split_done_undone(todos: &[TodoDTO]) -> (Vec<TodoDTO>, Vec<TodoDTO>) {
    todos.iter().cloned().partition(|i| i.completed)
}
