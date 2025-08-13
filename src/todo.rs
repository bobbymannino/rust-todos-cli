use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::{self, Read, Write},
};

pub struct TodoList {
    todos: Vec<Todo>,
}

impl TodoList {
    /// Reads todos from the "todos.json" file and returns them as a Vec<Todo>. If
    /// the file does not exist it will create an empty one. Sorted by date created
    /// ascending
    pub fn new() -> Result<Self, io::Error> {
        let mut todos_str = String::new();

        if !File::open("todos.json").is_ok() {
            File::create("todos.json")?.write_all(b"[]")?;
        }

        let mut file = File::open("todos.json")?;
        file.read_to_string(&mut todos_str)?;
        let mut todos: Vec<Todo> = serde_json::from_str(&todos_str)?;

        todos.sort_by(|a, b| {
            a.created_at()
                .timestamp_millis()
                .cmp(&b.created_at().timestamp_millis())
        });

        Ok(Self { todos })
    }

    /// Saves the todos to "todos.json"
    pub fn save(&self) {
        let todos_str = serde_json::to_string(&self.todos).unwrap();
        File::create("todos.json")
            .unwrap()
            .write_all(todos_str.as_bytes())
            .unwrap();
    }

    /// List todos
    pub fn list(&self) {
        for todo in &self.todos {
            println!("#{}: {}", todo.id(), todo.title());
        }
    }

    pub fn todos_mut(&mut self) -> &mut Vec<Todo> {
        &mut self.todos
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Todo {
    id: u32,
    title: String,
    body: Option<String>,
    done_at: Option<DateTime<Utc>>,
    created_at: DateTime<Utc>,
}

impl Todo {
    pub fn new(title: String, body: Option<String>) -> Self {
        Self {
            id: 0,
            title,
            body,
            done_at: None,
            created_at: Utc::now(),
        }
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn body(&self) -> Option<&str> {
        self.body.as_deref()
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn is_done(&self) -> bool {
        self.done_at.is_some()
    }

    pub fn toggle_done(&mut self) {
        self.done_at = match self.done_at {
            Some(_) => None,
            None => Some(Utc::now()),
        };
    }
}
