#![allow(dead_code, unused_variables)]

use types::traits::persistency::*;

use crate::todo_dto::TodoDTO;

#[derive(Debug, Default, Clone)]
struct TodoPersistencyDummy {
    pub data: Vec<TodoDTO>,
}

pub fn create_todo_persistecy_dummy() -> Box<dyn IPeristency<TodoDTO>> {
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
    fn load(&self) -> Vec<TodoDTO> {
        println!("Dummy todo data loaded.");
        self.data.clone()
    }

    fn save(&self, data: Vec<TodoDTO>) {
        println!("Dummy todo data saved.");
    }
}
