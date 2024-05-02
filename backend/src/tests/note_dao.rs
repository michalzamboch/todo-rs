#![allow(dead_code, unused_variables, unused_imports)]

use std::{error::Error, ops::Deref, sync::*, thread, time::Duration};

use crate::{note_dto::*, note_persistency_json::*, types::traits::persistency::*};

const TEST_ID: u32 = 0;
const TEST_TITLE: &str = "Test title";
const TEST_PATH: &str = "src/tests/files/todo_test_data.json";

pub fn create_new_test() -> Arc<RwLock<Vec<NoteDTO>>> {
    let v: Vec<NoteDTO> = vec![];
    let calculation_result = Arc::new(RwLock::new(v));
    let tmp = calculation_result.clone();

    let note = NoteDTO::new(TEST_ID, TEST_TITLE);
    tokio::spawn(async move {
        let mut locked = tmp.write().unwrap();
        *locked = vec![note];
    });

    calculation_result
}

#[tokio::test]
async fn read_test_data() {
    let res = create_new_test();

    tokio::time::sleep(Duration::from_millis(10)).await;
    let data = res.read();

    assert!(data.is_ok_and(|i| i.len() == 1));
}
