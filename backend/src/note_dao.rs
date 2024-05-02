#![allow(dead_code, unused_variables)]

use std::{error::Error, sync::*};

use crate::{
    note_dto::*,
    note_persistency_json::*,
    paths::*,
    types::traits::{dao::*, persistency::*},
};

pub struct NoteDAOFactory {}

impl NoteDAOFactory {
    /*
    pub fn create_test(path: &str) -> NoteDAO {
        let persistency = Arc::new(create_note_json_persistency(path));
        let empty: Vec<NoteDTO> = vec![];
        let calculation_result = Arc::new(Mutex::new(empty));

        let loadable = persistency.clone();
        let value_future = async move {
            let val = loadable.load().await;
            val.unwrap()
        };

        let result_for_thread = calculation_result.clone();
        let handle = tokio::spawn(async move {
            let result = value_future.await;
            let mut mutex_lock = result_for_thread.lock().unwrap();
            *mutex_lock = result;
        });

        let result = NoteDAO {
            todos: calculation_result,
            persistency: persistency.clone(),
        };

        result
    }
     */

    pub fn create_loaded() -> DaoThreadSafeRef<NoteDTO> {
        let dao = Self::create();
        let thread_dao = dao.clone();

        tokio::spawn(async move {
            let note = NoteDTO::new(1, "Notes");

            let binding = thread_dao.read().unwrap();
            let res = binding.persistency.load();
            thread_dao.write().unwrap().todos = Arc::new(Mutex::new(vec![note]));
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
            todos: Arc::new(Mutex::new(vec![])),
            persistency: Arc::new(create_note_json_persistency(JSON_TODO_FILEPATH)),
        }
    }
}

#[derive(Debug)]
pub struct NoteDAO {
    todos: Arc<Mutex<Vec<NoteDTO>>>,
    persistency: Arc<Box<dyn IPeristencyAsync<NoteDTO>>>,
}

impl NoteDAO {
    async fn reload(&self) {}
}

impl IDaoThreadSafe<NoteDTO> for NoteDAO {
    fn select_by(&self, id: u32) -> Option<NoteDTO> {
        todo!()
    }

    fn get_all(&self) -> Vec<NoteDTO> {
        let res = self.todos.lock().unwrap();
        res.clone()
    }

    fn insert_row(&mut self, item: &NoteDTO) -> Result<(), Box<dyn std::error::Error>> {
        let clone = self.persistency.clone();
        let data = vec![NoteDTO::new(0, "Sent note.")];
        tokio::spawn(async move {
            let x = clone.save(&data).await;
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
