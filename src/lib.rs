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
    list_name: String,
    todos: Vec<Todo>,
}

#[wasm_bindgen]
impl TodoList {
    pub fn new(list_name: &str) -> Self {
        Self {
            list_name: list_name.to_string(),
            todos: Vec::new(),
        }
    }

    pub fn list_name(&self) -> String {
        self.list_name.clone()
    }
    pub fn add(&mut self, todo_title: String) {
        let id = self.get_id();
        let todo = Todo::new(id, &todo_title);
        self.todos.push(todo);
    }

    pub fn get_by_id(&self, id: u32) -> Option<Todo>{
        self.todos.iter().find(|todo| todo.id() == id).cloned()
    }
    pub fn get_by_index(&self, index: usize) -> Option<Todo>{
        self.todos.get(index).cloned()
    }

    pub fn remove(&mut self, id: u32) {
        self.todos.retain(|todo| todo.id() != id);
    }

    pub fn len(&self) -> u32 {
        self.todos.len() as u32
    }

    pub fn set_completed(&mut self, id: u32, completed: bool) {
        if let Some(todo) = self.todos.iter_mut().find(|todo| todo.id() == id) {
            todo.set_completed(completed);
        }
    }
}

impl TodoList{
    fn get_id(&self) -> u32 {
        self.todos.len() as u32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_todo() {
        let todo = Todo::new(1, &"test".to_string());
        assert_eq!(todo.id(), 1);
        assert_eq!(todo.title(), "test");
        assert_eq!(todo.completed(), false);
    }

    #[test]
    fn test_todo_list() {
        let mut todo_list = TodoList::new("test");
        assert_eq!(todo_list.list_name(), "test");
        assert_eq!(todo_list.len(), 0);

        todo_list.add("test1".to_string());
        assert_eq!(todo_list.len(), 1);

        todo_list.add("test2".to_string());
        assert_eq!(todo_list.len(), 2);

        let todo = todo_list.get_by_id(0).unwrap();
        assert_eq!(todo.id(), 0);
        assert_eq!(todo.title(), "test1");
        assert_eq!(todo.completed(), false);

        let todo = todo_list.get_by_index(1).unwrap();
        assert_eq!(todo.id(), 1);
        assert_eq!(todo.title(), "test2");
        assert_eq!(todo.completed(), false);

        todo_list.set_completed(0, true);
        let todo = todo_list.get_by_id(0).unwrap();
        assert_eq!(todo.id(), 0);
        assert_eq!(todo.title(), "test1");
        assert_eq!(todo.completed(), true);

        todo_list.remove(0);
        assert_eq!(todo_list.len(), 1);
        let todo = todo_list.get_by_id(0);
        assert!(todo.is_none());
        let todo = todo_list.get_by_index(0).unwrap();
        assert_eq!(todo.id(), 1);
        assert_eq!(todo.title(), "test2");
        assert_eq!(todo.completed(), false);
    }
}