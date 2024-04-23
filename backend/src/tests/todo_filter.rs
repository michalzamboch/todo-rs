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
