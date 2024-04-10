#![allow(dead_code, unused_variables)]

use crate::todo_dto::*;
use types::traits::persistency::*;

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
        ],
    };

    Box::new(result)
}

impl TodoPersistencyDummy {}

impl IPeristency<TodoDTO> for TodoPersistencyDummy {
    fn load(&self) -> Result<Vec<TodoDTO>, Box<dyn Error>> {
        Ok(self.data.clone())
    }

    fn save(&self, data: Vec<TodoDTO>) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}
