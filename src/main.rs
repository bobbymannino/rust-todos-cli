mod todo;
mod todos;

use crate::todos::{get_all_todos, list_todos, save_todos};
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut todos = get_all_todos().expect("Failed to get todos");

    if args.len() == 1 {
        print!("Use one of the commands to see something happen");
    } else if args.len() == 2 {
        if args[1] == "list" {
            let mut todos = todos.clone();
            todos.retain(|todo| !todo.is_done());
            list_todos(&todos);
        } else if args[1] == "list-done" {
            let mut todos = todos.clone();
            todos.retain(|todo| todo.is_done());
            list_todos(&todos);
        } else if args[1] == "list-all" {
            list_todos(&todos);
        }
    } else if args.len() == 3 {
        if args[1] == "done" {
            let todo_id: u32 = args[2].parse().expect("Invalid todo ID");
            let todo = todos.iter_mut().find(|todo| todo.id() == todo_id);

            if let Some(todo) = todo {
                todo.toggle_done();
                save_todos(&todos);
            } else {
                println!("No todo with that ID")
            }
        } else if args[1] == "remove" {
            let todo_id: u32 = args[2].parse().expect("Invalid todo ID");
            todos.retain(|todo| todo.id() != todo_id);
            save_todos(&todos);
        }
    }
}
