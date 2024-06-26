#![allow(dead_code, unused_imports)]

use crate::todo_dto::*;

const TEST_ID: u32 = 0;
const TEST_TITLE: &str = "Test title";

#[test]
fn create_todo() {
    let todo = TodoDTO::new(TEST_ID, TEST_TITLE);

    assert_eq!(todo.id(), TEST_ID);
    assert_eq!(todo.title, TEST_TITLE);
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

#[test]
fn update_equal_todos() {
    let mut todo = TodoDTO::new(TEST_ID, TEST_TITLE);
    let other = TodoDTO::new(TEST_ID, "New title");

    let result = todo.update_from_equal(other.clone());
    assert!(result.is_ok());
    assert_eq!(todo.title, other.title);
}

#[test]
fn update_not_equal_todos() {
    let mut todo = TodoDTO::new(TEST_ID, TEST_TITLE);
    let other = TodoDTO::new(2, TEST_TITLE);

    let result = todo.update_from_equal(other.clone());
    assert!(result.is_err());
    assert_ne!(todo.id(), other.id());
}
