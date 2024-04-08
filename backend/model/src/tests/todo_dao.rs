#![allow(dead_code, unused_imports)]

use types::enums::todo_type::TodoType;

use crate::todo_dao::*;
use crate::todo_dto::*;

const TEST_ID: u32 = 1;
const TEST_TITLE: &str = "Test title";

#[test]
fn create_empty_todo_dao() {
    let todo_dao = TodoDAOFactory::create_dummy_ref();
    let count = todo_dao.count();
    let max_id = todo_dao.max_id();

    assert_eq!(count, 0);
    assert_eq!(max_id, 0);
}

#[test]
fn size_after_insert() {
    let todo_dao = TodoDAOFactory::create_dummy_ref();
    let result = todo_dao.insert_row(TodoDTO::new(TEST_ID, TEST_TITLE));

    assert_eq!(todo_dao.count(), 1);
    assert_eq!(todo_dao.max_id(), 1);
    assert!(result.is_ok());
}

#[test]
fn insert_row() {
    let todo_dao = TodoDAOFactory::create_dummy_ref();
    todo_dao.insert_row(TodoDTO::new(TEST_ID, TEST_TITLE)).expect("Could not insert row");
    let result = todo_dao.select_by(TEST_ID);

    assert!(result.is_some());
    assert_eq!(result.unwrap().id(), TEST_ID);
}

#[test]
fn insert_existing_row() {
    let todo_dao = TodoDAOFactory::create_dummy_ref();
    let todo = TodoDTO::new(TEST_ID, TEST_TITLE);
    let first = todo_dao.insert_row(todo.clone());
    let second = todo_dao.insert_row(todo);

    assert_eq!(todo_dao.count(), 1);
    assert!(first.is_ok());
    assert!(second.is_err());
}

#[test]
fn select_some_item() {
    let todo_dao = TodoDAOFactory::create_filled_dummy_ref();
    let todo = todo_dao.select_by(1);

    assert!(todo.is_some())
}

#[test]
fn select_nothing() {
    let todo_dao = TodoDAOFactory::create_filled_dummy_ref();
    let todo = todo_dao.select_by(100);

    assert!(todo.is_none())
}

#[test]
fn update_existing_row() {
    let todo_dao = TodoDAOFactory::create_dummy_ref();
    let todo = TodoDTO::new(TEST_ID, TEST_TITLE);

    let mut inserted = todo_dao.insert_row(todo).expect("Missing todo.");

    let new_title = "New title";
    inserted.set_title(new_title);

    let updated = todo_dao.update_row(inserted);

    assert!(updated.is_ok());
    assert_eq!(updated.unwrap().title(), new_title);
}