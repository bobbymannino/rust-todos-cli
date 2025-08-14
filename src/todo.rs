use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::{
    fs,
    io::{self, ErrorKind},
    path::Path,
    process,
};

const TODO_FILE: &str = "todos.json";

pub struct TodoList {
    todos: Vec<Todo>,
}

impl TodoList {
    /// Reads todos from the "todos.json" file and returns them as a Vec<Todo>. If
    /// the file does not exist it will create an empty one. Sorted by date created
    /// ascending
    pub fn new() -> Result<Self, io::Error> {
        let todos = if Path::new(TODO_FILE).exists() {
            let content = fs::read_to_string(TODO_FILE)?;
            if content.trim().is_empty() {
                Vec::<Todo>::new()
            } else {
                serde_json::from_str(&content)
                    .map_err(|e| io::Error::new(ErrorKind::InvalidData, e))?
            }
        } else {
            fs::write(TODO_FILE, "[]")?;
            Vec::new()
        };

        let mut sorted_todos = todos;
        sorted_todos.sort_by_key(|todo| todo.created_at.timestamp_millis());

        Ok(Self {
            todos: sorted_todos,
        })
    }

    /// Saves the todos to "todos.json"
    pub fn save(&self) -> Result<(), io::Error> {
        let content = serde_json::to_string_pretty(&self.todos)
            .map_err(|e| io::Error::new(ErrorKind::InvalidData, e))?;
        fs::write(TODO_FILE, content)
    }

    pub fn save_or_exit(&self) {
        if let Err(e) = self.save() {
            eprintln!("Failed to save todos: {}", e);
            process::exit(1);
        }
    }

    /// List all todos
    pub fn list(&self) {
        if self.todos.is_empty() {
            println!("No todos found.");
            return;
        }

        for todo in &self.todos {
            self.print_todo(todo);
        }
    }

    /// List todos filtered by completion status
    pub fn list_filtered(&self, show_done: bool) {
        let filtered: Vec<_> = self
            .todos
            .iter()
            .filter(|todo| todo.is_done() == show_done)
            .collect();

        if filtered.is_empty() {
            let status = if show_done { "completed" } else { "pending" };
            println!("No {} todos found.", status);
            return;
        }

        for todo in filtered {
            self.print_todo(todo);
        }
    }

    fn print_todo(&self, todo: &Todo) {
        let status = if todo.is_done() { "✓" } else { " " };
        println!("[{}] #{}: {}", status, todo.id, todo.title);
        if let Some(body) = &todo.body {
            println!("    {}", body);
        }
    }

    /// Toggle todo completion status by ID
    pub fn toggle_todo_done(&mut self, id: u32) -> bool {
        if let Some(todo) = self.todos.iter_mut().find(|t| t.id == id) {
            todo.toggle_done();
            true
        } else {
            false
        }
    }

    /// Remove todo by ID
    pub fn remove_todo(&mut self, id: u32) -> bool {
        let initial_len = self.todos.len();
        self.todos.retain(|todo| todo.id != id);
        self.todos.len() < initial_len
    }

    /// Update todo by ID
    pub fn update_todo(&mut self, id: u32, title: &str, body: Option<&str>) -> bool {
        if let Some(todo) = self.todos.iter_mut().find(|t| t.id == id) {
            todo.update(title.to_string(), body.map(|s| s.to_string()));
            true
        } else {
            false
        }
    }

    /// Returns the next ID for a new Todo. Will get the highest existing ID and add 1
    fn get_next_id(&self) -> u32 {
        self.todos.iter().map(|todo| todo.id).max().unwrap_or(0) + 1
    }

    /// Create and add a new todo
    pub fn new_todo(&mut self, title: &str, body: Option<&str>) {
        let id = self.get_next_id();
        let todo = Todo::new(title.to_string(), body.map(|s| s.to_string()), id);
        self.todos.push(todo);
    }
}

#[derive(Serialize, Deserialize, Clone)]
struct Todo {
    id: u32,
    title: String,
    body: Option<String>,
    done_at: Option<DateTime<Utc>>,
    created_at: DateTime<Utc>,
}

impl Todo {
    pub fn new(title: String, body: Option<String>, id: u32) -> Self {
        Self {
            id,
            title,
            body,
            done_at: None,
            created_at: Utc::now(),
        }
    }

    pub fn update(&mut self, title: String, body: Option<String>) {
        self.title = title;
        self.body = body;
    }

    pub fn is_done(&self) -> bool {
        self.done_at.is_some()
    }

    pub fn toggle_done(&mut self) {
        self.done_at = if self.is_done() {
            None
        } else {
            Some(Utc::now())
        };
    }
}
