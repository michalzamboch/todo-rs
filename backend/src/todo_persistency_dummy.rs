#![allow(dead_code, unused_variables)]

use crate::todo_dto::*;
use crate::types::traits::persistency::*;

use std::error::Error;

#[derive(Debug, Default, Clone)]
struct TodoPersistencyDummy {
    data: Vec<TodoDTO>,
}

pub fn create_todo_persistency_dummy() -> Box<dyn IPeristency<TodoDTO>> {
    let result = TodoPersistencyDummy {
        data: vec![
            TodoDTO::new(1, "Learn rust"),
            TodoDTO::new(2, "Learn egui"),
            TodoDTO::new(3, "Learn slint"),
            TodoDTO::new(4, "Learn rust"),
            TodoDTO::new(5, "Learn egui"),
            TodoDTO::new(6, "Learn slint"),
            TodoDTO::new(7, "Learn rust"),
            TodoDTO::new(8, "Learn egui"),
            TodoDTO::new(9, "Learn slint"),
            TodoDTO::new(10, "Learn rust"),
            TodoDTO::new(11, "Learn egui"),
            TodoDTO::new(12, "Learn X"),
        ],
    };

    Box::new(result)
}

impl TodoPersistencyDummy {}

impl IPeristency<TodoDTO> for TodoPersistencyDummy {
    fn load(&self) -> Result<Vec<TodoDTO>, Box<dyn Error>> {
        Ok(self.data.clone())
    }

    fn save(&self, data: &[TodoDTO]) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}

#[derive(Debug, Default, Clone)]
struct EmptyPersistency;

pub fn create_empty_todo_persistency() -> Box<dyn IPeristency<TodoDTO>> {
    Box::new(EmptyPersistency)
}

impl IPeristency<TodoDTO> for EmptyPersistency {
    fn load(&self) -> Result<Vec<TodoDTO>, Box<dyn Error>> {
        Ok(vec![])
    }

    fn save(&self, data: &[TodoDTO]) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}
