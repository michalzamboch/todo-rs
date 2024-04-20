#![allow(dead_code)]

use backend::{todo_dto::*, types::traits::dao::*};

use crate::todo_pipeline::*;

#[derive(Debug, Clone)]
pub struct TodoViewHandler {
    dao: DaoRef<TodoDTO>,
    pub pipeline: Box<TodoPipeline>,
    pub cache: Vec<TodoDTO>,
    pub current_todo: Option<TodoDTO>,
    pub new_todo_title: String,
}

pub fn create_todo_handler(todo_dao: DaoRef<TodoDTO>) -> Box<TodoViewHandler> {
    let dao = todo_dao.clone();
    let pipeline = create_todo_pipeline(dao.clone());
    let todos = dao.get_all();

    let handler = TodoViewHandler {
        dao,
        pipeline,
        cache: todos,
        current_todo: None,
        new_todo_title: "".to_owned(),
    };

    Box::new(handler)
}

impl TodoViewHandler {
    pub fn get_cache(&self) -> &[TodoDTO] {
        self.cache.as_slice()
    }

    pub fn update(&mut self) {
        let update = self.pipeline.execute();
        if update {
            self.cache = self.dao.get_all();
        }
    }
}
