mod todo;
mod todos;

use crate::todos::{get_all_todos, list_todos};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let todos = get_all_todos().expect("Failed to get todos");

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
    }
}
