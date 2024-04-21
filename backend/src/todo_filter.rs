use ::serde::*;
use std::fmt;

#[derive(Debug, Default, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct FilterTodosBy {
    pub id_range: Option<(u32, u32)>,
    pub title: Option<String>,
    pub completed: Option<bool>,
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

#[derive(Debug, Default, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct TodoFilterBuilder {
    filter_by: FilterTodosBy,
}

impl TodoFilterBuilder {
    pub fn new() -> Self {
        Self {
            filter_by: FilterTodosBy::default(),
        }
    }
    pub fn id_range(mut self, range: (u32, u32)) -> TodoFilterBuilder {
        self.filter_by.id_range = Some(range);
        self
    }

    pub fn title(mut self, title: &str) -> TodoFilterBuilder {
        self.filter_by.title = Some(title.to_owned());
        self
    }

    pub fn completed(mut self, completed: bool) -> TodoFilterBuilder {
        self.filter_by.completed = Some(completed);
        self
    }

    pub fn build(&self) -> FilterTodosBy {
        self.filter_by.clone()
    }
}
