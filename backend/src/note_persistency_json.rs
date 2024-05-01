use async_trait::async_trait;

use crate::types::traits::persistency::*;

use crate::note_dto::*;

use std::error::Error;
use std::fs::File;
use std::io::{BufReader, BufWriter, Write};

#[derive(Debug, Default, Clone)]
struct NotePersistencyJson {
    filepath: String,
}

pub fn create_note_json_persistency(path: &str) -> Box<dyn IPeristencyAsync<NoteDTO>> {
    let result = NotePersistencyJson {
        filepath: path.to_owned(),
    };

    Box::new(result)
}

#[async_trait]
impl IPeristencyAsync<NoteDTO> for NotePersistencyJson {
    async fn load(&self) -> Result<Vec<NoteDTO>, Box<dyn Error>> {
        let file = File::open(self.filepath.as_str())?;
        let reader = BufReader::new(file);

        let result = serde_json::from_reader(reader)?;
        Ok(result)
    }

    async fn save(&self, data: &[NoteDTO]) -> Result<(), Box<dyn Error>> {
        let file = File::create(self.filepath.as_str())?;
        let mut writer = BufWriter::new(file);
        serde_json::to_writer(&mut writer, &data)?;
        writer.flush()?;

        Ok(())
    }
}
