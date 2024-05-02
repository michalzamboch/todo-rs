#![allow(dead_code, unused_variables)]

use std::{error::Error, ops::Deref, sync::*};

use crate::{note_dto::*, note_persistency_json::*, types::traits::persistency::*};

pub async fn create_new_test(
    path: &str,
) -> Arc<Mutex<Option<Result<Vec<NoteDTO>, BoxedSendError>>>> {
    let calculation_result = Arc::new(Mutex::new(None));
    let tmp = calculation_result.clone();

    let p = create_note_json_persistency(path);
    let x = p.load().await;

    let mut locked = tmp.lock().unwrap();
    *locked = Some(x);
    calculation_result
}

const TEST_ID: u32 = 0;
const TEST_TITLE: &str = "Test title";
const TEST_PATH: &str = "src/tests/files/todo_test_data.json";

#[tokio::test]
async fn read_test_data() {
    let res = create_new_test(TEST_PATH).await;
    let data = res.lock();
    match data {
        Ok(x) => {
        },
        Err(_) => todo!(),
    }

}
