#![allow(dead_code, unused_imports)]

use std::rc::Rc;

use types::traits::dao::IDao;
use super::todo_dto::*;
use super::todo_dao::*;

#[derive(Debug)]
struct ModelHandler {
    todos: Rc<Box<dyn IDao<TodoDTO>>>,
}

fn create_new_handler_dummy() -> ModelHandler {
    ModelHandler {
        todos: create_todo_dao_dummy_ref(),
    }
}

impl ModelHandler {
    fn test(&self) {
    }
}