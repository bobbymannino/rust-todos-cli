mod todo;

use crate::todo::TodoList;
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    let mut todo_list = match TodoList::new() {
        Ok(list) => list,
        Err(e) => {
            eprintln!("Failed to load todos: {}", e);
            process::exit(1);
        }
    };

    match args.len() {
        0 => println!("Usage: todo [list|list-done|list-all|add|done|remove|update] [args...]"),

        1 => match args[0].as_str() {
            "list" => todo_list.list_filtered(false),
            "list-done" => todo_list.list_filtered(true),
            "list-all" => todo_list.list(),
            _ => {
                eprintln!("Invalid command: {}", args[0]);
                process::exit(1);
            }
        },

        2 => match args[0].as_str() {
            "add" => {
                todo_list.new_todo(&args[1], None);
                todo_list.save_or_exit();
            }
            "done" => {
                if let Ok(id) = args[1].parse::<u32>() {
                    if todo_list.toggle_todo_done(id) {
                        todo_list.save_or_exit();
                    } else {
                        eprintln!("No todo with ID {}", id);
                        process::exit(1);
                    }
                } else {
                    eprintln!("Invalid todo ID: {}", args[1]);
                    process::exit(1);
                }
            }
            "remove" => {
                if let Ok(id) = args[1].parse::<u32>() {
                    if todo_list.remove_todo(id) {
                        todo_list.save_or_exit();
                    } else {
                        eprintln!("No todo with ID {}", id);
                        process::exit(1);
                    }
                } else {
                    eprintln!("Invalid todo ID: {}", args[1]);
                    process::exit(1);
                }
            }
            _ => {
                eprintln!("Invalid command or arguments");
                process::exit(1);
            }
        },

        3 => match args[0].as_str() {
            "add" => {
                todo_list.new_todo(&args[1], Some(&args[2]));
                todo_list.save_or_exit();
            }
            "update" => {
                if let Ok(id) = args[1].parse::<u32>() {
                    if todo_list.update_todo(id, &args[2], None) {
                        todo_list.save_or_exit();
                    } else {
                        eprintln!("No todo with ID {}", id);
                        process::exit(1);
                    }
                } else {
                    eprintln!("Invalid todo ID: {}", args[1]);
                    process::exit(1);
                }
            }
            _ => {
                eprintln!("Invalid command or arguments");
                process::exit(1);
            }
        },

        4 => match args[0].as_str() {
            "update" => {
                if let Ok(id) = args[1].parse::<u32>() {
                    if todo_list.update_todo(id, &args[2], Some(&args[3])) {
                        todo_list.save_or_exit();
                    } else {
                        eprintln!("No todo with ID {}", id);
                        process::exit(1);
                    }
                } else {
                    eprintln!("Invalid todo ID: {}", args[1]);
                    process::exit(1);
                }
            }
            _ => {
                eprintln!("Invalid command or arguments");
                process::exit(1);
            }
        },

        _ => {
            eprintln!("Too many arguments");
            process::exit(1);
        }
    }
}
