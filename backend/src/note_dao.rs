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

            let binding = thread_dao.read().unwrap();
            let res = binding.persistency.load();
            thread_dao.write().unwrap().notes = Arc::new(Mutex::new(vec![note]));
        });

        dao.clone()
    }

    pub fn create() -> Arc<RwLock<NoteDAO>> {
        let base = Self::create_base();
        let with_lock = RwLock::new(base);

        Arc::new(with_lock)
    }

    pub fn create_base() -> NoteDAO {
        NoteDAO {
            notes: Arc::new(Mutex::new(vec![])),
            persistency: Arc::new(create_note_json_persistency(JSON_NOTE_FILEPATH)),
        }
    }
}

#[derive(Debug)]
pub struct NoteDAO {
    notes: Arc<Mutex<Vec<NoteDTO>>>,
    persistency: Arc<Box<dyn IPeristencyAsync<NoteDTO>>>,
}

impl ILoadable<NoteDTO> for NoteDAO {
    fn load(&self) -> Result<(), BoxedSendError> {
        let thread_persistency = self.persistency.clone();
        let thread_data = self.notes.clone();

        let handler = tokio::spawn(async move {
            let data = thread_persistency.load().await;

            if let Ok(vec) = data {
                let mut target = thread_data.lock().unwrap();
                *target = vec;
            }
        });

        Ok(())
    }
}

impl IDaoThreadSafe<NoteDTO> for NoteDAO {
    fn select_by(&self, id: u32) -> Option<NoteDTO> {
        todo!()
    }

    fn get_all(&self) -> Vec<NoteDTO> {
        let res = self.notes.lock().unwrap();
        res.clone()
    }

    fn insert_row(&mut self, item: &NoteDTO) -> Result<(), Box<dyn std::error::Error>> {
        let mut writer = self.notes.lock().unwrap();
        writer.push(item.clone());

        let thread_persistency = self.persistency.clone();
        let thread_data = self.notes.clone();

        tokio::spawn(async move {
            let data = thread_data.lock().unwrap().clone();
            let result = thread_persistency.save(&data).await;
        });

        Ok(())
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
