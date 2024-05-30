#![allow(dead_code)]

use backend::{todo_dto::*, todo_filter::*, types::traits::dao::*};

use super::{todo_cache::TodoCache, todo_pipeline::*};

#[derive(Debug, Clone)]
pub struct TodoViewHandler {
    dao: DaoRef<TodoDTO>,
    pipeline: Box<TodoPipeline>,
}

pub fn create_todo_handler(todo_dao: DaoRef<TodoDTO>) -> Box<TodoViewHandler> {
    let dao = todo_dao.clone();
    let pipeline = create_prepared_todo_pipeline(dao.clone());

    let handler = TodoViewHandler { dao, pipeline };

    Box::new(handler)
}

impl TodoViewHandler {
    pub fn push_command(&mut self, cmd: PipelineCommand) {
        self.pipeline.push(cmd);
    }

    pub fn push_commands(&mut self, cmds: &[PipelineCommand]) {
        self.pipeline.push_array(cmds);
    }

    pub fn process_pipeline(&mut self, cache: &mut TodoCache) {
        let update = self.execute_pipeline();
        if update {
            self.data_update(cache);
        }
    }

    fn execute_pipeline(&mut self) -> bool {
        self.pipeline.execute()
    }

    fn data_update(&self, cache: &mut TodoCache) {
        let all_todos = self.dao.get_all();
        let divided = split_done_undone(&all_todos);
        cache.done = divided.0;

        if cache.activate_search {
            let search_term = &cache.search_term;
            let filtered = FilterTodosBy::new().title(search_term).filter(&divided.1);
            cache.undone = filtered;
        } else {
            cache.undone = divided.1;
        }
    }
}
