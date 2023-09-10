mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct Todo {
    id: u32,
    title: String,
    completed: bool,
}

impl Todo{
    pub fn new(id: u32, title: String, completed: bool) -> Self {
        Self {
            id,
            title,
            completed,
        }
    }
}

#[wasm_bindgen]
impl Todo{
    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn title(&self) -> String {
        self.title.clone()
    }

    pub fn completed(&self) -> bool {
        self.completed
    }

    pub fn set_completed(&mut self, completed: bool) {
        self.completed = completed;
    }
}

#[wasm_bindgen]
pub struct TodoList {
    todos: Vec<Todo>,
}

#[wasm_bindgen]
impl TodoList{
    pub fn new() -> Self {
        Self {
            todos: Vec::new(),
        }
    }

    pub fn add(&mut self, title: String) {
        let id = self.todos.len() as u32;
        let todo = Todo::new(id,title, false);
        self.todos.push(todo);
    }

    pub fn get(&self, id: u32) -> Option<Todo> {
        self.todos.get(id as usize).cloned()
    }

    pub fn remove(&mut self, id: u32) {
        self.todos.remove(id as usize);
    }

    pub fn todos(&self) -> *const Todo {
        self.todos.as_ptr()
    }
}