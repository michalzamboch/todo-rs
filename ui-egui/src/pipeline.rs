#![allow(dead_code)]

use std::collections::*;

use backend::todo_dto::*;

#[derive(Debug, Default, Clone, Hash, PartialEq, Eq)]
pub enum PipelineCommand {
    #[default]
    None,
    DeleteTodo(TodoDTO),
    CreateTodo(TodoDTO),
    UpdateTodo(TodoDTO),
}

#[derive(Debug, Default, Clone, Hash, PartialEq, Eq)]
pub struct ViewPipeline {
    commands: VecDeque<PipelineCommand>,
}

impl ViewPipeline {
    pub fn execute(&mut self) {
        for cmd in self.commands.iter() {
            self.execute_command(cmd);
        }
        self.commands.clear();
    }

    fn execute_command(&self, cmd: &PipelineCommand) {
        match cmd {
            PipelineCommand::None => (),
            PipelineCommand::DeleteTodo(_todo) => todo!(),
            PipelineCommand::CreateTodo(_todo) => todo!(),
            PipelineCommand::UpdateTodo(_todo) => todo!(),
        }
    }

    pub fn push(&mut self, cmd: PipelineCommand) {
        self.commands.push_back(cmd);
    }
}
