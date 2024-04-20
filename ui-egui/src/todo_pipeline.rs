#![allow(dead_code)]

use std::collections::*;

use backend::todo_dto::*;
use backend::types::traits::dao::*;

#[derive(Debug, Default, Clone, Hash, PartialEq, Eq)]
pub enum PipelineCommand {
    #[default]
    None,
    Delete(TodoDTO),
    Create(TodoDTO),
    CreateUsingTitle(String),
    Update(TodoDTO),
}

#[derive(Debug, Clone)]
pub struct TodoPipeline {
    dao: DaoRef<TodoDTO>,
    commands: VecDeque<PipelineCommand>,
}

pub fn create_todo_pipeline(dao: DaoRef<TodoDTO>) -> Box<TodoPipeline> {
    let pipeline = TodoPipeline {
        dao: dao.clone(),
        commands: VecDeque::new(),
    };

    Box::new(pipeline)
}

impl TodoPipeline {
    pub fn execute(&mut self) -> bool {
        let clear = !self.commands.is_empty();
        let mut update = false;

        for cmd in self.commands.iter() {
            update |= self.execute_command(cmd);
        }

        if clear {
            self.commands.clear();
        }

        update
    }

    fn execute_command(&self, cmd: &PipelineCommand) -> bool {
        match cmd {
            PipelineCommand::None => false,
            PipelineCommand::Delete(todo) => {
                self.dao
                    .remove_row(todo.id())
                    .expect("Unable to remove row");
                true
            }
            PipelineCommand::Create(todo) => {
                self.dao.insert_row(todo).expect("Unable to insert row");
                true
            }
            PipelineCommand::CreateUsingTitle(title) => {
                let id = self.dao.max_id() + 1;
                let todo = TodoDTO::new(id, title);
                self.dao.insert_row(&todo).expect("Unable to insert row");
                true
            }
            PipelineCommand::Update(todo) => {
                self.dao.update_row(todo).expect("Unable to update row");
                true
            }
        }
    }

    pub fn push(&mut self, cmd: PipelineCommand) {
        self.commands.push_back(cmd);
    }
}
