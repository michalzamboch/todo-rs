
#![allow(dead_code, unused_imports)]

use types::enums::todo_type::TodoType;

use crate::{todo_dto::*, todo_persistency_dummy::*};

const TEST_ID: u32 = 0;
const TEST_TITLE: &str = "Test title";


#[test]
fn save_dummy() {
    let persistent = create_todo_persistency_dummy();
    let data = persistent.load();
    persistent.save(data);
}
