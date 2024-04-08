#![allow(dead_code, unused_imports)]

use types::enums::todo_type::TodoType;

use crate::todo_dto::*;

const TEST_ID: u32 = 0;
const TEST_TITLE: &str = "Test title";

#[test]
fn create_todo() {
    let todo = TodoDTO::new(TEST_ID, TEST_TITLE);

    assert_eq!(todo.id(), TEST_ID);
    assert_eq!(todo.title(), TEST_TITLE);
    assert_eq!(todo.todo_type(), TodoType::Generic);
    assert_eq!(todo.completed(), false);
}

#[test]
fn todo_equal_by_id() {
    let todo = TodoDTO::new(TEST_ID, TEST_TITLE);
    let other = TodoDTO::new(TEST_ID, "New title");

    assert_eq!(todo.equal_by_id(&other), true);
}

#[test]
fn todo_not_equal_by_id() {
    let todo = TodoDTO::new(TEST_ID, TEST_TITLE);
    let other = TodoDTO::new(2, TEST_TITLE);

    assert_eq!(todo.equal_by_id(&other), false);
}
