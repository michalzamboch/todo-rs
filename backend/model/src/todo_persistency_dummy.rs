#![allow(dead_code, unused_variables)]

use types::traits::persistency::*;

use crate::todo_dto::TodoDTO;

use std::error::Error;
use std::fs::File;
use std::io::{BufWriter, Write};

const JSON_NAME: &str = "todo_persistency.json";

#[derive(Debug, Default, Clone)]
struct TodoPersistencyDummy {
    pub data: Vec<TodoDTO>,
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
        println!("Dummy todo data loaded.");
        let result = self.data.clone();
        Ok(result)
    }

    fn save(&self, data: Vec<TodoDTO>) -> Result<(), Box<dyn Error>> {
        let file = File::create(JSON_NAME)?;
        let mut writer = BufWriter::new(file);
        serde_json::to_writer(&mut writer, &data)?;
        writer.flush()?;
        Ok(())
    }
}
