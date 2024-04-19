#![allow(dead_code, unused_imports)]

use std::rc::Rc;

use super::todo_dao::*;
use super::todo_dto::*;
use types::traits::dao::*;

#[derive(Debug)]
pub struct ModelHandler {
    todos: DaoRef<TodoDTO>,
}

pub fn create_new_handler() -> Box<ModelHandler> {
    let model = ModelHandler {
        todos: TodoDAOFactory::create_filled_dummy_ref(),
    };

    Box::new(model)
}

impl ModelHandler {
    pub fn todos(&self) -> DaoRef<TodoDTO> {
        self.todos.clone()
    }
}
