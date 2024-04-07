#![allow(dead_code, unused_imports)]

use std::rc::Rc;

use super::todo_dao::*;
use super::todo_dto::*;
use types::traits::dao::IDao;

#[derive(Debug)]
struct ModelHandler {
    todos: Rc<Box<dyn IDao<TodoDTO>>>,
}

fn create_new_handler_dummy() -> ModelHandler {
    ModelHandler {
        todos: TodoDAOFactory::create_filled_dummy_ref(),
    }
}

impl ModelHandler {}
