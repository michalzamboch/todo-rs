#![allow(dead_code, unused_variables)]

use std::sync::*;

use crate::{
    note_dto::*,
    note_persistency_json::*,
    paths::*,
    types::traits::{dao::*, persistency::*},
};

pub struct NoteDAOFactory {}

impl NoteDAOFactory {
    pub fn create_loaded() -> DaoThreadSafeRef<NoteDTO> {
        let dao = Self::create();
        let thread_dao = dao.clone();

        tokio::spawn(async move {
            let note = NoteDTO::new(1, "Notes");
            thread_dao.write().unwrap().todos = vec![note];
        });

        dao.clone()
    }

    fn create() -> Arc<RwLock<NoteDAO>> {
        let base = Self::create_base();
        let with_lock = RwLock::new(base);

        Arc::new(with_lock)
    }

    fn create_base() -> NoteDAO {
        NoteDAO {
            todos: vec![],
            persistency: create_note_json_persistency(JSON_TODO_FILEPATH),
        }
    }
}

#[derive(Debug)]
struct NoteDAO {
    todos: Vec<NoteDTO>,
    persistency: Box<dyn IPeristencyAsync<NoteDTO>>,
}

impl IDaoThreadSafe<NoteDTO> for NoteDAO {
    fn select_by(&self, id: u32) -> Option<NoteDTO> {
        todo!()
    }

    fn get_all(&self) -> Vec<NoteDTO> {
        todo!()
    }

    fn insert_row(&mut self, item: &NoteDTO) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }

    fn update_row(&mut self, item: &NoteDTO) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }

    fn remove_row(&mut self, id: u32) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }

    fn count(&self) -> u32 {
        todo!()
    }

    fn max_id(&self) -> u32 {
        todo!()
    }

    fn exists(&self, id: u32) -> bool {
        todo!()
    }
}
