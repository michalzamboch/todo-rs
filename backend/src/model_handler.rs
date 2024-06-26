#![allow(dead_code)]

use super::todo_dao::*;
use super::todo_dto::*;
use crate::types::traits::dao::*;

#[derive(Debug, Clone)]
pub struct ModelHandler {
    todos: DaoRef<TodoDTO>,
}

pub fn create_new_handler() -> Box<ModelHandler> {
    let model = ModelHandler {
        todos: TodoDAOFactory::create(),
    };

    Box::new(model)
}

impl ModelHandler {
    pub fn todos(&self) -> DaoRef<TodoDTO> {
        self.todos.clone()
    }
}
