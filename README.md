# Rust Todos CLI (WIP)

A very simple todos app.

## Usage

| Description                | Example                                                 | Status |
| -------------------------- | ------------------------------------------------------- | ------ |
| List **not** done todos    | `cargo run -- list`                                     | ✅     |
| List done todos            | `cargo run -- list-done`                                | ✅     |
| List all todos             | `cargo run -- list-all`                                 | ✅     |
| Toggle done status on todo | `cargo run -- done <todo id>`                           | ✅     |
| Remove todo                | `cargo run -- remove <todo id>`                         | ✅     |
| Add todo                   | `cargo run -- add <todo text> <todo body>`              | ✅     |
| Update todo                | `cargo run -- update <todo id> <todo text> <todo body>` | 🏗️     |
