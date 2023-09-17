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
    pub fn new(id: u32, title: &String) -> Self {
        Self {
            id,
            title: title.clone(),
            completed: false,
        }
    }
}

#[wasm_bindgen]
impl Todo{
    pub fn id(&self) ->u32{
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
impl TodoList {
    pub fn new() -> Self {
        Self {
            todos: Vec::new(),
        }
    }

    pub fn add(&mut self, todo_title: String) {
        let id = self.get_id();
        let todo = Todo::new(id, &todo_title);
        self.todos.push(todo);
    }

    pub fn get(&self, id: u32) -> Option<Todo>{
        self.todos.iter().find(|todo| todo.id() == id).cloned()
    }

    pub fn remove(&mut self, id: u32) {
        self.todos.retain(|todo| todo.id() != id);
    }

    pub fn len(&self) -> u32 {
        self.todos.len() as u32
    }
}

impl TodoList{
    fn get_id(&self) -> u32 {
        self.todos.len() as u32
    }
}