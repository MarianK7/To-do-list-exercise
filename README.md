# Todo List CLI

A simple command-line interface for managing a todo list written in Rust. The program stores the tasks in a JSON file named `todo.json` and provides the following subcommands:

## Subcommands
* `help`: Displays a help message with a list of available subcommands.
* `add`: Adds a new task to the todo list. The syntax for this command is `add <task description>`.
* `complete`: Marks a task as complete. The syntax for this command is `complete <index>`, where `<index>` is the index of the task to mark as complete.
* `list`: Lists all of the tasks in the todo list.

## Usage
To use the program, clone the repository and run the following command in the terminal:

```
cargo run <subcommand>
```

Replace `<subcommand>` with one of the available subcommands listed above.

## Dependencies
The program uses the following dependencies:
* clap: for defining the command-line interface
* serde: for serializing and deserializing the Task struct
* serde_json: for pretty-printing JSON
* std::error: for error handling
* std::fs: for file I/O
* std::io: for efficient writing to the todo.json file
