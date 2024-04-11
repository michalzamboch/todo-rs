#![allow(dead_code, unused_imports)]

use std::{error::Error, fs, result};

use types::enums::todo_type::TodoType;

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

fn compare_files(file_path1: &str, file_path2: &str) -> Result<bool, Box<dyn Error>> {
    let contents1 = fs::read(file_path1)?;
    let contents2 = fs::read(file_path2)?;

    Ok(contents1 == contents2)
}
