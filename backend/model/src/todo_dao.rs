#![allow(dead_code, unused_variables)]

use std::{error::Error, rc::Rc};

use crate::{todo_dto::*, todo_persistency_dummy::*};
use types::traits::{dao::IDao, persistency::IPeristency};

#[derive(Debug)]
struct TodoDAO {
    todos: Vec<TodoDTO>,
    persistency: Box<dyn IPeristency<TodoDTO>>,
}

fn create_todo_dao_dummy() -> TodoDAO {
    let persistency = create_todo_persistecy_dummy();
    let loaded_todos = persistency.load();
    
    TodoDAO {
        todos: loaded_todos,
        persistency,
    }
}

pub fn create_todo_dao_dummy_ref() -> Rc<Box<dyn IDao<TodoDTO>>> {
    let dao = create_todo_dao_dummy();
    let boxed_dao = Box::new(dao);

    Rc::new(boxed_dao)
}

impl TodoDAO {
    fn test(&mut self) {
        self.todos.push(TodoDTO::default());
    }
}

impl IDao<TodoDTO> for TodoDAO {
    fn select_by(&self, id: u32) -> Option<TodoDTO> {
        todo!()
    }

    fn insert_row(&mut self, item: TodoDTO) -> Option<TodoDTO> {
        self.todos.push(item.clone());
        Some(item)
    }

    fn update_row(&mut self, item: TodoDTO) -> Result<TodoDTO, Box<dyn Error>> {
        todo!()
    }

    fn count(&self) -> u32 {
        todo!()
    }

    fn max_id(&self) -> u32 {
        todo!()
    }
}
