#![allow(dead_code, unused_imports)]

use crate::tests::todo_helpers::*;
use crate::todo_filter::*;

use crate::todo_dto::*;

#[test]
fn filter_by_id() {
    let todos = get_test_todos();
    let filtered = FilterTodosBy::new().id_range((1, 2)).filter(&todos);

    assert_eq!(filtered.len(), 2);
    assert_eq!(filtered[0].id(), 1);
    assert_eq!(filtered[1].id(), 2);
}

#[test]
fn filter_completed() {
    let mut todos: Vec<TodoDTO> = get_test_todos().into();
    todos[5].completed = true;
    todos[6].completed = true;

    let filtered = FilterTodosBy::new().completed(true).filter(&todos);

    assert_eq!(filtered.len(), 2);
    assert_eq!(filtered[0].id(), 5);
    assert_eq!(filtered[1].id(), 6);
}

#[test]
fn filter_by_title() {
    let todos: Vec<TodoDTO> = get_test_todos().into();
    let filtered = FilterTodosBy::new().title("X").filter(&todos);

    assert_eq!(filtered.len(), 1);
    assert_eq!(filtered[0].id(), 11);
    assert_eq!(filtered[0].title, "Learn X");
}

#[test]
fn complex_filter() {
    let mut todos: Vec<TodoDTO> = get_test_todos().into();
    todos[0].completed = true;
    todos[1].completed = true;
    let filtered = FilterTodosBy::new()
        .completed(true)
        .title("Y")
        .filter(&todos);

    assert_eq!(filtered.len(), 1);
    assert_eq!(filtered[0].id(), 0);
    assert_eq!(filtered[0].title, "Learn Y");
}

#[test]
fn filter_none() {
    let todos: Vec<TodoDTO> = get_test_todos().into();
    let count = todos.len();
    let filtered = FilterTodosBy::new().filter(&todos);

    assert_eq!(filtered.len(), count);
}
