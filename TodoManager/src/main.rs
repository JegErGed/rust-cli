mod data_structs;
use std::path::{Path, PathBuf};
use std::env;
use data_structs::DB;
fn main() {
    use text_io::read;
    println!("Welcome to your command line ToDo manager!\n\n");
    let mut db: DB = DB::new();
    let exe_path: PathBuf = env::current_exe().expect("Failed to get current exe path");
    let path = exe_path
        .parent()
        .expect("Failed to get parent directory")
        .join("todo.db");
    let path = path
        .to_str()
        .expect("Failed to convert path to string")
        .to_string();
    if Path::new(&path).exists() {
        db = DB::reload_from_file(&path)
    }

    loop {
        db.print_db();
        println!("Type (+) to add a task.\nType (-) to remove a task.\nType (*) to remove all completed tasks.\nType (/) to change the status of a task\nType 'x' to exit.\n");
        let choice: String = read!("{}\n");

        if choice.is_empty() {
            println!("Invalid input. Please enter a command.");
            continue;
        }

        let ch: char = choice.chars().next().unwrap();

        match ch {
            '+' => {
                db.user_add_create_element();
            }
            '-' => {
                println!("Choose the index of the task you want to remove:");
                let task_index: usize = read!();
                if task_index > 0 && task_index <= db.0.len() {
                    db.remove_task(task_index - 1);
                } else {
                    println!("Invalid task index.");
                }
            }
            '/' => {
                println!("Choose the index of the task you want to change:");
                let task_index_input: String = read!("{}\n");
                let task_index_input = task_index_input.trim();

                if let Ok(task_index) = task_index_input.parse::<usize>() {
                    if task_index > 0 && task_index <= db.0.len() {
                        db.0[task_index - 1].print_element();
                        println!("Is this task completed? (y/n)");
                        let answer: String = read!("{}\n");
                        let ans: char = answer.chars().next().unwrap_or(' ');

                        let status = match ans {
                            'y' | 'Y' => true,
                            'n' | 'N' => false,
                            _ => {
                                println!("Invalid input. Please enter 'y' or 'n'.");
                                continue; // Skip the rest of the loop iteration and ask for input again
                            }
                        };

                        db.update_task_status(task_index - 1, status);
                    } else {
                        println!("Invalid task index.");
                    }
                } else {
                    println!("Invalid input. Please enter a valid number.");
                }
            }
            '*' => {
                db.remove_completed_tasks();
            }
            'x' => {
                db.save_to_file(path);

                return;
            }
            _ => println!("We don't recognize your input, try again."),
        }
    }
}
