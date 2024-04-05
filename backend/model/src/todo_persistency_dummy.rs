#![allow(dead_code, unused_variables)]

use types::traits::persistency::*;

use crate::todo_dto::TodoDTO;

#[derive(Debug, Default, Clone)]
struct TodoPersistencyDummy {
    pub data: Vec<TodoDTO>,
}

impl TodoPersistencyDummy {
    pub fn create() -> Box<dyn IPeristency<TodoDTO>> {
        let result = TodoPersistencyDummy {
            data: vec![
                TodoDTO::new(1, "Learn rust".to_owned()),
                TodoDTO::new(2, "Learn egui".to_owned()),
                TodoDTO::new(3, "Learn slint".to_owned()),
            ],
        };

        Box::new(result)
    }
}

impl IPeristency<TodoDTO> for TodoPersistencyDummy {
    fn load(&self) -> Vec<TodoDTO> {
        println!("Dummy todo data loaded.");
        self.data.clone()
    }

    fn save(&self, data: Vec<TodoDTO>) {
        println!("Dummy todo data saved.");
    }
}
