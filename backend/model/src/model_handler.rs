#![allow(dead_code, unused_imports)]

use std::rc::Rc;

use super::todo_dao::*;
use super::todo_dto::*;
use types::traits::dao::*;

#[derive(Debug)]
pub struct ModelHandler {
    todos: Rc<Box<dyn IDao<TodoDTO>>>,
}

pub fn create_new_handler() -> ModelHandler {
    ModelHandler {
        todos: TodoDAOFactory::create_filled_dummy_ref(),
    }
}

impl ModelHandler {
    pub fn todos(&self) -> Rc<Box<dyn IDao<TodoDTO>>> {
        self.todos.clone()
    }
}
