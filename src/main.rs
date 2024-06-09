#[derive(Clone, Debug, PartialEq)]
enum Val {
    Name(String),
    Desc(String),
    Status(bool),
}

#[derive(Clone, Debug, PartialEq)]
struct Elm(Vec<Val>);

impl Elm {
    fn new(name: String, desc: String) -> Elm {
        let mut element = Elm(Vec::new());
        element.0.push(Val::Name(name));
        element.0.push(Val::Desc(desc));
        element.0.push(Val::Status(false));
        element
    }

    fn set_status(&mut self, status: bool) {
        // Create a new vector to hold the modified entries
        let mut new_entries = Vec::new();

        for entry in &self.0 {
            match entry {
                Val::Name(name) => new_entries.push(Val::Name(name.clone())),
                Val::Desc(desc) => new_entries.push(Val::Desc(desc.clone())),
                Val::Status(_) => (), // Skip the existing status
            }
        }

        // Add the new status entry
        new_entries.push(Val::Status(status));

        // Replace the original vector with the new vector
        self.0 = new_entries;
    }

    fn print_element(self: &Elm) {
        for entry in &self.0 {
            match entry {
                Val::Name(name) => println!("  Name: {}", name),
                Val::Desc(desc) => println!("  Description: {}", desc),
                Val::Status(status) => {
                    let status_str = if *status { "Completed!" } else { "Not completed!" };
                    println!("  Status: {}", status_str);
                },
            }
        }
    }
    fn is_completed(&self) -> bool {
        for entry in &self.0 {
            if let Val::Status(status) = entry {
                return *status;
            }
        }
        false
    }

}

#[derive(Clone, Debug, PartialEq)]
struct DB(Vec<Elm>);
impl DB {
    fn new() -> DB {
        DB(Vec::new())
    }
    fn remove_task(&mut self, index: usize) {
        if index < self.0.len() {
            self.0.remove(index);
        } else {
            println!("Invalid task index.");
        }
    }
    fn print_db(&mut self) {
        for (index, element) in self.0.iter().enumerate() {
            println!("Task {}:", index + 1);
            element.print_element();
        }
    }
    fn update_task_status(&mut self, task_index: usize, status: bool) {
        self.0[task_index].set_status(status);
    }
    fn add_create_element(&mut self, name: String, desc: String) {
        self.0.push(Elm::new(name, desc));
    }
    fn user_add_create_element(&mut self){
        use text_io::read;
        println!("Type the task name: ");
        let name: String = {
            let temp: String = read!("{}\n");
            temp.trim().to_string()
        };
        println!("Type the task description: ");
        let desc: String = {
            let temp: String = read!("{}\n");
            temp.trim().to_string()
        };
        self.add_create_element(name, desc)
    }
    fn remove_completed_tasks(&mut self) {
        let mut indices = Vec::new();
        for (index, task) in self.0.iter().enumerate() {
            if task.is_completed() {
                indices.push(index);
            }
        }
        for index in indices.iter().rev() {
            self.remove_task(*index);
        }
    }
    
}








fn main() {
    use text_io::read;
    println!("Welcome to your command line ToDo manager!\n\n");
    let mut db: DB = DB::new();
    loop {
        println!("Type (+) to add a task.\nType (-) to remove a task.\nType (*) to remove all completed tasks.\nType (/) to change the status of a task\nType 'x' to exit.\n");
        db.print_db();
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
                let task_index: usize = read!();
                if task_index > 0 && task_index <= db.0.len() {
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
                }

            '*' => {
                db.remove_completed_tasks();
            }
            'x' => break,
            _ => println!("We don't recognize your input, try again."),
        }
    }
}
