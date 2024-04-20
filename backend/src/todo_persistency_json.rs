#![allow(dead_code)]

use crate::types::traits::persistency::*;

use crate::todo_dto::TodoDTO;

use std::error::Error;
use std::fs::File;
use std::io::{BufReader, BufWriter, Write};

#[derive(Debug, Default, Clone)]
struct TodoPersistencyJson {
    filepath: String,
}

pub fn create_todo_json_persistency(path: &str) -> Box<dyn IPeristency<TodoDTO>> {
    let result = TodoPersistencyJson {
        filepath: path.to_owned(),
    };

    Box::new(result)
}

impl TodoPersistencyJson {}

impl IPeristency<TodoDTO> for TodoPersistencyJson {
    fn load(&self) -> Result<Vec<TodoDTO>, Box<dyn Error>> {
        let file = File::open(self.filepath.as_str())?;
        let reader = BufReader::new(file);

        let result = serde_json::from_reader(reader)?;
        Ok(result)
    }

    fn save(&self, data: Vec<TodoDTO>) -> Result<(), Box<dyn Error>> {
        let file = File::create(self.filepath.as_str())?;
        let mut writer = BufWriter::new(file);
        serde_json::to_writer(&mut writer, &data)?;
        writer.flush()?;

        Ok(())
    }
}
