#![allow(dead_code, unused_imports, unused_variables)]

use test_case::test_case;

use crate::types::traits::dao::*;

use crate::todo_dao::*;
use crate::todo_dto::*;

const TEST_ID: u32 = 1;
const TEST_TITLE: &str = "Test title";

#[test]
fn create_empty_todo_dao() {
    let todo_dao = TodoDAOFactory::create_empty_dummy();
    let count = todo_dao.count();
    let max_id = todo_dao.max_id();

    assert_eq!(count, 0);
    assert_eq!(max_id, 0);
}

#[test]
fn size_after_insert() {
    let todo_dao = TodoDAOFactory::create_empty_dummy();
    let result = todo_dao.insert_row(&TodoDTO::new(TEST_ID, TEST_TITLE));

    assert_eq!(todo_dao.count(), 1);
    assert_eq!(todo_dao.max_id(), 1);
    assert!(result.is_ok());
}

#[test]
fn insert_row() {
    let todo_dao = TodoDAOFactory::create_empty_dummy();
    todo_dao
        .insert_row(&TodoDTO::new(TEST_ID, TEST_TITLE))
        .expect("Could not insert row");
    let result = todo_dao.select_by(TEST_ID);

    assert!(result.is_some());
    assert_eq!(result.unwrap().id(), TEST_ID);
}

#[test]
fn insert_existing_row() {
    let todo_dao = TodoDAOFactory::create_empty_dummy();
    let todo = TodoDTO::new(TEST_ID, TEST_TITLE);
    let first = todo_dao.insert_row(&todo);
    let second = todo_dao.insert_row(&todo);

    assert_eq!(todo_dao.count(), 1);
    assert!(first.is_ok());
    assert!(second.is_err());
}

#[test]
fn select_some_item() {
    let todo_dao = TodoDAOFactory::create_filled_dummy();
    let todo = todo_dao.select_by(1);

    assert!(todo.is_some())
}

#[test]
fn select_nothing() {
    let todo_dao = TodoDAOFactory::create_filled_dummy();
    let todo = todo_dao.select_by(100);

    assert!(todo.is_none())
}

#[test]
fn update_existing_row() {
    let todo_dao = TodoDAOFactory::create_empty_dummy();
    let mut todo = TodoDTO::new(TEST_ID, TEST_TITLE);

    todo_dao.insert_row(&todo).expect("Unable to insert row.");

    let new_title = "New title";
    todo.title = new_title.to_owned();

    let updated = todo_dao.update_row(&todo);
    assert!(updated.is_ok());

    let updated_row = todo_dao.select_by(TEST_ID);
    assert!(updated_row.is_some_and(|item| item.title == new_title));
}

#[test]
fn remove_row() {
    let todo_dao = TodoDAOFactory::create_empty_dummy();

    for i in 0..3 {
        let todo = TodoDTO::new(i, TEST_TITLE);
        todo_dao.insert_row(&todo).expect("Unable to insert row");
    }

    assert_eq!(todo_dao.count(), 3);

    let result = todo_dao.remove_row(0);
    assert!(result.is_ok());
    assert_eq!(todo_dao.count(), 2);
}

#[test]
fn removal_of_non_existing_row() {
    let todo_dao = TodoDAOFactory::create_empty_dummy();

    let result = todo_dao.remove_row(0);
    assert!(result.is_err());
    assert_eq!(todo_dao.count(), 0);
}

#[test]
fn get_all() {
    let todo_dao = TodoDAOFactory::create_empty_dummy();
    let count = 3;

    for i in 0..count {
        let todo = TodoDTO::new(i, TEST_TITLE);
        todo_dao.insert_row(&todo).expect("Unable to insert row");
    }

    assert_eq!(todo_dao.count(), count);

    let todos = todo_dao.get_all();
    assert_eq!(todos.len() as u32, count);
}

#[test]
fn get_empty_vec() {
    let todo_dao = TodoDAOFactory::create_empty_dummy();
    let todos = todo_dao.get_all();

    assert_eq!(todos.len(), 0);
    assert_eq!(todo_dao.count(), 0);
}

#[test_case(TodoDAOFactory::create_empty_dummy(); "Regular DAO")]
fn test_case_example(dao: DaoRef<TodoDTO>) {}
