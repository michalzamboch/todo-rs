#![allow(dead_code, unused_imports, unused_variables)]

use test_case::test_case;

use crate::types::traits::dao::*;

use crate::note_dao::*;
use crate::note_dto::*;

const TEST_ID: u32 = 0;
const TEST_TITLE: &str = "Test title";
const TEST_PATH: &str = "src/tests/files/todo_test_data.json";
