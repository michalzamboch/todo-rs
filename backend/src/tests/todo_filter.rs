#![allow(dead_code, unused_imports)]

use crate::todo_filter::*;
use crate::types::enums::todo_type::*;

use crate::todo_dto::*;

fn get_test_todos() -> Box<[TodoDTO]> {
    let result: [TodoDTO; 12] = [
        TodoDTO::new(1, "Learn rust"),
        TodoDTO::new(2, "Learn egui"),
        TodoDTO::new(3, "Learn slint"),
        TodoDTO::new(4, "Learn rust"),
        TodoDTO::new(5, "Learn egui"),
        TodoDTO::new(6, "Learn slint"),
        TodoDTO::new(7, "Learn rust"),
        TodoDTO::new(8, "Learn egui"),
        TodoDTO::new(9, "Learn slint"),
        TodoDTO::new(10, "Learn rust"),
        TodoDTO::new(11, "Learn egui"),
        TodoDTO::new(12, "Learn X"),
    ];
    Box::new(result)
}

#[test]
fn filter_by_id() {
    let todos = get_test_todos();
    let filtered = FilterTodosBy::new().id_range((1, 2)).filter(&todos);

    assert_eq!(filtered.len(), 2);
    assert_eq!(filtered[0].id(), 1);
    assert_eq!(filtered[1].id(), 2);
}
