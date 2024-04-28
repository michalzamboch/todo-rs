#![allow(dead_code)]

use std::collections::*;

use backend::todo_dto::*;
use backend::types::traits::dao::*;

#[derive(Debug, Default, Clone, Hash, PartialEq, Eq)]
pub enum PipelineCommand {
    #[default]
    None,
    Load,
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

pub fn create_prepared_todo_pipeline(dao: DaoRef<TodoDTO>) -> Box<TodoPipeline> {
    let mut pipeline = create_todo_pipeline(dao.clone());
    pipeline.push(PipelineCommand::Load);
    pipeline
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
            PipelineCommand::Load => true,
            PipelineCommand::Delete(todo) => {
                self.remove_action(todo);
                true
            }
            PipelineCommand::Create(todo) => {
                self.create_action(todo);
                true
            }
            PipelineCommand::CreateUsingTitle(title) => {
                self.create_using_title_action(title);
                true
            }
            PipelineCommand::Update(todo) => {
                self.update_action(todo);
                true
            }
        }
    }

    fn update_action(&self, todo: &TodoDTO) {
        let result = self.dao.update_row(todo);
        match result {
            Ok(_) => println!("Updated todo: {}", todo),
            Err(e) => println!("Update of todo failed: {}", e),
        }
    }

    fn create_using_title_action(&self, title: &str) {
        let id = self.dao.max_id() + 1;
        let todo = TodoDTO::new(id, title);

        let result = self.dao.insert_row(&todo);
        match result {
            Ok(_) => println!("Created new todo with name: {}", title),
            Err(e) => println!("Created new todo with name failed: {}", e),
        }
    }

    fn create_action(&self, todo: &TodoDTO) {
        let result = self.dao.insert_row(todo);
        match result {
            Ok(_) => println!("Created todo: {}", todo),
            Err(e) => println!("Creating new todo failed: {}", e),
        }
    }

    fn remove_action(&self, todo: &TodoDTO) {
        let result = self.dao.remove_row(todo.id());
        match result {
            Ok(_) => println!("Removed todo: {}", todo),
            Err(e) => println!("Remove of todo failed: {}", e),
        }
    }

    pub fn push(&mut self, cmd: PipelineCommand) {
        self.commands.push_back(cmd);
    }
}
