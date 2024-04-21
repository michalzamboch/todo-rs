#![allow(dead_code, unused_variables)]

use std::{cell::RefCell, cmp::*, error::Error, ops::Deref, rc::Rc};

use crate::{
    paths::*,
    todo_dto::*,
    todo_persistency_dummy::*,
    todo_persistency_json::*,
    types::traits::{dao::IDao, persistency::IPeristency},
};

#[derive(Debug)]
struct TodoDAO {
    todos: RefCell<Vec<TodoDTO>>,
    persistency: Box<dyn IPeristency<TodoDTO>>,
}

pub struct TodoDAOFactory {}

impl TodoDAOFactory {
    pub fn create() -> Rc<Box<dyn IDao<TodoDTO>>> {
        let dao = Self::create_loaded();
        let boxed_dao = Box::new(dao);

        Rc::new(boxed_dao)
    }

    fn create_loaded() -> TodoDAO {
        let persistency = create_todo_json_persistency(JSON_TODO_FILEPATH);
        let loaded_todos = persistency.load().unwrap_or_default();

        TodoDAO {
            todos: RefCell::new(loaded_todos),
            persistency,
        }
    }

    pub fn create_dummy_ref() -> Rc<Box<dyn IDao<TodoDTO>>> {
        let dao = Self::create_empty_dummy();
        let boxed_dao = Box::new(dao);

        Rc::new(boxed_dao)
    }

    fn create_empty_dummy() -> TodoDAO {
        TodoDAO {
            todos: RefCell::new(vec![]),
            persistency: create_todo_persistency_dummy(),
        }
    }

    pub fn create_filled_dummy_ref() -> Rc<Box<dyn IDao<TodoDTO>>> {
        let dao = Self::create_filled_dummy();
        let boxed_dao = Box::new(dao);

        Rc::new(boxed_dao)
    }

    fn create_filled_dummy() -> TodoDAO {
        let persistency = create_todo_persistency_dummy();
        let loaded_todos = persistency.load().unwrap_or_default();

        TodoDAO {
            todos: RefCell::new(loaded_todos),
            persistency,
        }
    }
}

impl IDao<TodoDTO> for TodoDAO {
    fn select_by(&self, id: u32) -> Option<TodoDTO> {
        for i in self.todos.borrow().iter() {
            if i.id() == id {
                return Some(i.clone());
            }
        }

        None
    }

    fn get_all(&self) -> Vec<TodoDTO> {
        self.todos.borrow().deref().clone()
    }

    fn insert_row(&self, item: &TodoDTO) -> Result<(), Box<dyn Error>> {
        if self.exists(item.id()) {
            return Err("Item already exists".into());
        }

        self.todos.borrow_mut().push(item.clone());
        Ok(())
    }

    fn update_row(&self, item: &TodoDTO) -> Result<(), Box<dyn Error>> {
        let found = self.todos.borrow().iter().position(|x| x.id() == item.id());
        match found {
            Some(index) => {
                self.todos.borrow_mut()[index].update_from_equal(item.clone())?;
                Ok(())
            }
            None => Err("Non existing id.".into()),
        }
    }

    fn remove_row(&self, id: u32) -> Result<(), Box<dyn Error>> {
        let found = self.todos.borrow().iter().position(|x| x.id() == id);
        match found {
            Some(index) => {
                self.todos.borrow_mut().remove(index);
                Ok(())
            }
            None => Err("Non existing id.".into()),
        }
    }

    fn count(&self) -> u32 {
        self.todos.borrow().len() as u32
    }

    fn max_id(&self) -> u32 {
        let values = self.todos.borrow();
        let mut max_id = 0;
        for v in values.iter() {
            max_id = max(max_id, v.id());
        }
        max_id
    }

    fn exists(&self, id: u32) -> bool {
        let values = self.todos.borrow();

        for v in values.iter() {
            if v.id() == id {
                return true;
            }
        }

        false
    }
}