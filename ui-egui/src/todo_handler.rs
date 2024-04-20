#![allow(dead_code)]

use backend::{todo_dto::*, types::traits::dao::*};

use crate::todo_pipeline::*;

#[derive(Debug, Clone)]
pub struct TodoViewHandler {
    dao: DaoRef<TodoDTO>,
    pub pipeline: Box<TodoPipeline>,
}

pub fn create_todo_handler(todo_dao: DaoRef<TodoDTO>) -> Box<TodoViewHandler> {
    let dao = todo_dao.clone();
    let pipeline = create_todo_pipeline(dao.clone());

    let handler = TodoViewHandler { dao, pipeline };

    Box::new(handler)
}
