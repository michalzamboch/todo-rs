use backend::todo_dto::*;

#[derive(Debug, Default)]
pub struct TodoCache {
    pub undone: Vec<TodoDTO>,
    pub done: Vec<TodoDTO>,
    pub current: TodoDTO,
    pub current_selected: bool,
    pub new_title: String,
    pub search_term: String,
    pub activate_search: bool,
}

pub fn create_todo_cache() -> Box<TodoCache> {
    let handler = TodoCache {
        ..Default::default()
    };

    Box::new(handler)
}
