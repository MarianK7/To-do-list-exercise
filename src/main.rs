use clap::{App, Arg}; // Import the required dependencies
use serde::{Deserialize, Serialize}; // Import serde to enable serialization and deserialization of the Task struct
use serde_json::to_string_pretty; // Import serde_json to enable pretty-printing of JSON
use std::error::Error; // Import Error to enable error handling
use std::fs::{File, OpenOptions}; // Import the required standard library modules for file I/O
use std::io::{BufWriter, Write}; // Import BufWriter and Write to enable efficient writing to the todo.json file // Import Error to enable error handling

// Define a struct to represent a task
#[derive(Serialize, Deserialize)]
struct Task {
    description: String, // A string describing the task
    completed: bool,     // A boolean indicating whether the task has been completed
}

// Define the main function
fn main() {
    // Define the command line interface using clap
    let matches = App::new("Todo List CLI")
        .version("0.1.0")
        .author("Marian Keszi")
        .about("A command-line interface for managing a todo list")
        // Define the subcommands
        .subcommand(
            App::new("add")
                .about("Add a new task to the todo list")
                // Define the arguments for the add subcommand
                .arg(
                    Arg::with_name("TASK")
                        .help("The task to add to the todo list")
                        .required(true),
                ),
        )
        .subcommand(
            App::new("complete").about("Mark a task as complete").arg(
                Arg::with_name("INDEX")
                    .help("The index of the task to mark as complete")
                    .required(true),
            ),
        )
        .subcommand(
            App::new("delete_completed").about("Delete all completed tasks from the todo list"),
        )
        .subcommand(App::new("list").about("List all of the tasks in the todo list"))
        .get_matches();

    let filename = "todo.json"; // Define the name of the file to store the tasks
    let file = OpenOptions::new()
        .read(true) // Open the file in read mode
        .write(true) // Open the file in write mode
        .create(true) // Create the file if it does not exist
        .open(filename) // Open the file with the specified filename
        .unwrap(); // Unwrap the result of the open operation, panicking if it fails

    // Deserialize the contents of the file into a vector of tasks, or create an empty vector if deserialization fails
    let mut tasks: Vec<Task> = match serde_json::from_reader(file) {
        Ok(tasks) => tasks,
        Err(_) => Vec::new(),
    };

    // Match the subcommand specified by the user and perform the appropriate action
    match matches.subcommand() {
        ("add", Some(sub_m)) => {
            let description = sub_m.value_of("TASK").unwrap();
            match add_task(&mut tasks, &filename, description) {
                Ok(_) => println!("Added task: {}", description),
                Err(e) => eprintln!("Error adding task: {}", e),
            }
            //println!("Added task: {}", description);
        }
        ("complete", Some(sub_m)) => {
            // Parse the index of the task to mark as complete
            let index = sub_m
                .value_of("INDEX")
                .unwrap()
                .parse::<usize>()
                .unwrap()
                .checked_sub(1)
                .expect("Index must be a positive integer");

            // Mark the task as complete
            match complete_task(&mut tasks, &filename, index) {
                Ok(_) => println!("Completed task: {}", tasks[index].description),
                Err(e) => eprintln!("Error completing task: {}", e),
            }
        }
        ("delete_completed", Some(_)) => {
            let initial_len = tasks.len();
            match delete_completed(&mut tasks, &filename) {
                Ok(_) => println!("Deleted {} completed tasks", initial_len - tasks.len()),
                Err(e) => eprintln!("Error deleting completed tasks: {}", e),
            }
        }
        ("list", Some(_)) => {
            if tasks.is_empty() {
                // Print message if there are no tasks
                println!("No tasks in the todo list");
            } else {
                // Print completed tasks
                println!("Completed tasks:");
                for (index, task) in tasks.iter().enumerate() {
                    if task.completed {
                        println!("{}. [{}] {}", index + 1, "x", task.description);
                    }
                }
                // Print uncompleted tasks
                println!("Uncompleted tasks:");
                for (index, task) in tasks.iter().enumerate() {
                    if task.completed == false {
                        println!("{}. [{}] {}", index + 1, " ", task.description);
                    }
                }
            }
        }
        _ => {
            // Print message if no subcommand was used
            println!("No subcommand was used, use -h, --help to see available subcommands");
        }
    }
}

// Define a function to save the tasks to the file
fn save_tasks(filename: &str, tasks: &Vec<Task>) {
    // Create a new buffered writer to write to the file
    let mut writer = BufWriter::new(File::create(filename).unwrap());
    // Write the tasks to the file in pretty-printed JSON format
    writer
        .write_all(to_string_pretty(tasks).unwrap().as_bytes())
        .unwrap();
}

fn add_task(
    tasks: &mut Vec<Task>,
    filename: &str,
    description: &str,
) -> Result<(), Box<dyn Error>> {
    // Create a new task with the specified description and mark it as uncompleted
    let task = Task {
        description: description.to_string(),
        completed: false,
    };
    // Add the task to the vector of tasks
    tasks.push(task);
    // Write the tasks to the file
    save_tasks(filename, tasks);
    Ok(())
}

fn complete_task(
    tasks: &mut Vec<Task>,
    filename: &str,
    index: usize,
) -> Result<(), Box<dyn Error>> {
    // Check if the index is valid
    if index >= tasks.len() {
        return Err("Index out of bounds".into());
    }

    // Check if the task is already completed
    if tasks[index].completed {
        println!("Task already completed");
    } else {
        // Mark the task as completed and write to file
        tasks[index].completed = true;
        save_tasks(&filename, &tasks);
    }

    Ok(())
}

fn delete_completed(tasks: &mut Vec<Task>, filename: &str) -> Result<(), Box<dyn Error>> {
    // Store the initial length of the vector before deletion
    let initial_len = tasks.len();

    // Use the retain method to remove all the completed tasks from the vector
    tasks.retain(|task| !task.completed);

    // If any completed tasks were removed, save the updated tasks to the file
    if tasks.len() < initial_len {
        save_tasks(filename, tasks);
    } else {
        // Otherwise, return an error message
        return Err("No completed tasks to delete".into());
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_save_tasks() {
        let filename = "test_save.json";
        let tasks = vec![
            Task {
                description: "Task 1".to_string(),
                completed: false,
            },
            Task {
                description: "Task 2".to_string(),
                completed: true,
            },
        ];

        // Save the tasks to the file
        save_tasks(filename, &tasks);

        // Read the contents of the file and deserialize them into a vector of tasks
        let file = File::open(filename).unwrap();
        let deserialized_tasks: Vec<Task> = serde_json::from_reader(file).unwrap();

        // Check that the deserialized tasks match the original tasks
        assert_eq!(tasks[0].description, deserialized_tasks[0].description);
        assert_eq!(tasks[0].completed, deserialized_tasks[0].completed);
        assert_eq!(tasks[1].description, deserialized_tasks[1].description);
        assert_eq!(tasks[1].completed, deserialized_tasks[1].completed);

        // Delete the file
        fs::remove_file(filename).unwrap();
    }

    #[test]
    fn test_add_task() {
        let filename = "test_add.json";
        let mut tasks = vec![Task {
            description: "Task 1".to_string(),
            completed: false,
        }];

        // Add a new task
        add_task(&mut tasks, filename, "Task 2").unwrap();

        // Read the contents of the file and deserialize them into a vector of tasks
        let file = File::open(filename).unwrap();
        let deserialized_tasks: Vec<Task> = serde_json::from_reader(file).unwrap();

        // Check that the deserialized tasks match the expected tasks
        let expected_tasks = vec![
            Task {
                description: "Task 1".to_string(),
                completed: false,
            },
            Task {
                description: "Task 2".to_string(),
                completed: false,
            },
        ];

        // Check that the deserialized tasks match the expected tasks
        assert_eq!(
            deserialized_tasks[0].description,
            expected_tasks[0].description
        );
        assert_eq!(deserialized_tasks[0].completed, expected_tasks[0].completed);
        assert_eq!(
            deserialized_tasks[1].description,
            expected_tasks[1].description
        );
        assert_eq!(deserialized_tasks[1].completed, expected_tasks[1].completed);

        // Delete the file
        fs::remove_file(filename).unwrap();
    }

    #[test]
    fn test_complete_task() {
        let filename = "test_complete.json";
        let mut tasks = vec![
            Task {
                description: "Task 1".to_string(),
                completed: false,
            },
            Task {
                description: "Task 2".to_string(),
                completed: true,
            },
        ];

        // Add a new task
        add_task(&mut tasks, filename, "Task 3").unwrap();

        // Complete added task
        complete_task(&mut tasks, filename, 2).unwrap();

        // Read the contents of the file and deserialize them into a vector of tasks
        let file = File::open(filename).unwrap();
        let deserialized_tasks: Vec<Task> = serde_json::from_reader(file).unwrap();

        let expected_tasks = vec![
            Task {
                description: "Task 1".to_string(),
                completed: false,
            },
            Task {
                description: "Task 2".to_string(),
                completed: true,
            },
            Task {
                description: "Task 3".to_string(),
                completed: true,
            },
        ];

        // Check that the first task is not completed
        assert_eq!(deserialized_tasks[0].completed, expected_tasks[0].completed);
        // Check that the added task is now completed
        assert_eq!(deserialized_tasks[2].completed, expected_tasks[2].completed);

        complete_task(&mut tasks, filename, 0).unwrap();

        // Check that the first task is completed
        assert_eq!(deserialized_tasks[0].completed, expected_tasks[0].completed);

        // Delete the file
        fs::remove_file(filename).unwrap();
    }

    #[test]
    fn test_delete_completed() {
        let filename = "test_delete_completed.json";
        let mut tasks = vec![
            Task {
                description: "Task 1".to_string(),
                completed: false,
            },
            Task {
                description: "Task 2".to_string(),
                completed: true,
            },
        ];

        // Add a new task
        add_task(&mut tasks, filename, "Task 3").unwrap();

        // Complete added task
        complete_task(&mut tasks, filename, 2).unwrap();

        // Delete completed tasks
        delete_completed(&mut tasks, filename).unwrap();

        // Read the contents of the file and deserialize them into a vector of tasks
        let file = File::open(filename).unwrap();
        let deserialized_tasks: Vec<Task> = serde_json::from_reader(file).unwrap();

        let expected_tasks = vec![Task {
            description: "Task 1".to_string(),
            completed: false,
        }];

        // Check that completed tasks are deleted
        assert_eq!(deserialized_tasks.len(), expected_tasks.len());
        // Check that the first task is not deleted
        assert_eq!(
            deserialized_tasks[0].description,
            expected_tasks[0].description
        );

        // Check that function raises error when there are no completed tasks to delete
        assert!(delete_completed(&mut tasks, filename).is_err());
        // Check that no tasks are deleted
        assert_eq!(deserialized_tasks.len(), expected_tasks.len());

        // Delete the file
        fs::remove_file(filename).unwrap();
    }
}
