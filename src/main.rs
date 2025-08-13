mod todo;

use crate::todo::{Todo, TodoList};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut todo_list = TodoList::new().expect("Failed to get todos");

    if args.len() == 1 {
        print!("Use one of the commands to see something happen");
    } else if args.len() == 2 {
        if args[1] == "list" {
            todo_list.todos_mut().retain(|todo| !todo.is_done());
            todo_list.list();
        } else if args[1] == "list-done" {
            todo_list.todos_mut().retain(|todo| todo.is_done());
            todo_list.list();
        } else if args[1] == "list-all" {
            todo_list.list();
        }
    } else if args.len() == 3 {
        if args[1] == "done" {
            let todo_id: u32 = args[2].parse().expect("Invalid todo ID");
            let todo = todo_list
                .todos_mut()
                .iter_mut()
                .find(|todo| todo.id() == todo_id);

            if let Some(todo) = todo {
                todo.toggle_done();
                todo_list.save();
            } else {
                println!("No todo with that ID")
            }
        } else if args[1] == "remove" {
            let todo_id: u32 = args[2].parse().expect("Invalid todo ID");
            todo_list.todos_mut().retain(|todo| todo.id() != todo_id);
            todo_list.save();
        }
    }
}
