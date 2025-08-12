use crate::todo::Todo;
use std::{
    fs::File,
    io::{self, Read, Write},
};

/// Reads todos from the "todos.json" file and returns them as a Vec<Todo>. If
/// the file does not exist it will create an empty one. Sorted by date created
/// ascending
pub fn get_all_todos() -> Result<Vec<Todo>, io::Error> {
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

    Ok(todos)
}

/// Saves the todos to "todos.json"
pub fn save_todos(todos: Vec<Todo>) {
    let todos_str = serde_json::to_string(&todos).unwrap();
    File::create("todos.json")
        .unwrap()
        .write_all(todos_str.as_bytes())
        .unwrap();
}

pub fn list_todos(todos: &Vec<Todo>) {
    for todo in todos {
        println!("#{}: {}", todo.id(), todo.title());
    }
}
