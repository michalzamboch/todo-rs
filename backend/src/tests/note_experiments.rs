#![allow(dead_code, unused_variables, unused_imports)]

use std::{error::Error, ops::Deref, sync::*, thread, time::Duration};

use crate::{note_dto::*, note_persistency_json::*, types::traits::persistency::*};

const TEST_ID: u32 = 0;
const TEST_TITLE: &str = "Test title";
const TEST_PATH: &str = "note_test.json";

#[tokio::test]
async fn read_test_data() {
    let res = new_basic_loader();

    tokio::time::sleep(Duration::from_millis(1)).await;
    let data = res.read();

    assert!(data.is_ok_and(|i| i.len() == 1));
}

fn new_basic_loader() -> Arc<RwLock<Vec<NoteDTO>>> {
    let empty: Vec<NoteDTO> = vec![];
    let calculation_result = Arc::new(RwLock::new(empty));
    let tmp = calculation_result.clone();

    let note = NoteDTO::new(TEST_ID, TEST_TITLE);
    tokio::spawn(async move {
        let mut locked = tmp.write().unwrap();
        *locked = vec![note];
    });

    calculation_result
}

#[tokio::test]
async fn load_data_from_persistency() {
    tokio::time::sleep(Duration::from_millis(10)).await;
}

fn persistency_loader(path: &str) -> Arc<RwLock<Vec<NoteDTO>>> {
    let empty: Vec<NoteDTO> = vec![];
    let result = Arc::new(RwLock::new(empty));
    let persistency = Arc::new(create_note_json_persistency(path));

    let thread_result = result.clone();
    let thread_persistency = persistency.clone();
    tokio::spawn(async move {
        let data = thread_persistency.load().await;

        if let Ok(vec) = data {
            let mut target = thread_result.write().unwrap();
            *target = vec;
        }
    });

    result
}

#[tokio::test]
async fn save_data_persistency() {
    persistency_saver(TEST_PATH);
    tokio::time::sleep(Duration::from_millis(10)).await;
}

fn persistency_saver(path: &str) {
    let note = NoteDTO::new(TEST_ID, TEST_TITLE);
    let data = vec![note];
    let persistency = Arc::new(create_note_json_persistency(path));

    let thread_persistency = persistency.clone();
    tokio::spawn(async move {
        let _ = thread_persistency.save(&data).await;
    });
}