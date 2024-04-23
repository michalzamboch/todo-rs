#![allow(dead_code, unused_imports)]

use std::{error::Error, fs, result};

use crate::tests::todo_helpers::*;
use crate::{todo_dto::*, todo_persistency_json::*};

const TEST_ID: u32 = 0;
const TEST_TITLE: &str = "Test title";
const TEST_PATH: &str = "src/tests/files/todo_test_data.json";

#[test]
fn load_data() {
    let persistency = create_todo_json_persistency(TEST_PATH);
    let result = persistency.load();

    assert!(result.is_ok());
    assert_eq!(result.unwrap().len(), 3);
}

#[test]
fn save_and_load() {
    let path = "todo_test.json";
    let persistency = create_todo_json_persistency(path);
    let data: Vec<TodoDTO> = get_test_todos().into();

    let save_result = persistency.save(&data);
    assert!(save_result.is_ok());

    let load_result = persistency.load();
    assert!(load_result.is_ok());

    assert_eq!(data, load_result.unwrap_or_default());
}
