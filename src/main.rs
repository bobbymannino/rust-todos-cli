mod todo;
mod todos;

use crate::todos::{get_todos, list_todos};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let todos = get_todos().expect("Failed to get todos");

    if args.len() > 1 {
        if args[1] == "list" {
            list_todos(&todos);
        }
    }
}
