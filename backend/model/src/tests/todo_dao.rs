#![allow(dead_code, unused_imports)]

use types::enums::todo_type::TodoType;
use uuid::*;

use crate::todo_dao::*;
use crate::todo_dto::*;

const TEST_ID: u32 = 0;
const TEST_TITLE: &str = "Test title";

#[test]
fn create_empty_todo_dao() {
    let todo_dao = TodoDAOFactory::create_dummy_ref();
    let count = todo_dao.count();
    let max_id = todo_dao.max_id();

    assert_eq!(count, 0);
    assert_eq!(max_id, 0);
}
