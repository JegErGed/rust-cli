#[derive(Clone, Debug, PartialEq)]
enum Val {
    Name(String),
    Desc(String),
    Status(bool),
}

#[derive(Clone, Debug, PartialEq)]
struct Elm(Vec<Val>);

fn create_element(name: String, desc: String) -> Elm {
    let mut element = Elm(Vec::new());
    element.0.push(Val::Name(name));
    element.0.push(Val::Desc(desc));
    element.0.push(Val::Status(false));
    element
}

fn set_status(element: &Elm, status: bool) -> Elm {
    let mut new_element = Vec::new();
    for entry in &element.0 {
        match entry {
            Val::Name(name) => new_element.push(Val::Name(name.clone())),
            Val::Desc(desc) => new_element.push(Val::Desc(desc.clone())),
            Val::Status(_) => (), // Skip the existing status
        }
    }
    new_element.push(Val::Status(status));
    Elm(new_element)
}

fn print_element(element: &Elm) {
    for entry in &element.0 {
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

fn print_db(db: &Vec<Elm>) {
    for (index, element) in db.iter().enumerate() {
        println!("Task {}:", index + 1);
        print_element(element);
    }
}

fn update_task_status(db: &mut Vec<Elm>, task_index: usize, status: bool) {
    let updated_task = set_status(&db[task_index], status);
    db[task_index] = updated_task;
}

fn user_input() -> Elm {
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
    create_element(name, desc)
}
fn main() {
    use text_io::read;
    println!("Welcome to your command line ToDo manager!\n\n");
    let mut db: Vec<Elm> = Vec::new();
    loop {
        println!("Type (+) to add a task.\nType (-) to remove a task.\nType (*) to remove all completed tasks.\nType (/) to change the status of a task\nType 'x' to exit.\n");
        print_db(&db);
        let choice: String = read!("{}\n");

        if choice.is_empty() {
            println!("Invalid input. Please enter a command.");
            continue;
        }

        let ch: char = choice.chars().next().unwrap();

        match ch {
            '+' => {
                let new_task: Elm = user_input();
                db.push(new_task.clone());
            }
            '/' => {
                println!("Choose the index of the task you want to change:");
                let task_index: usize = read!();
                println!("Enter the new status (true/false):");
                let status: bool = read!();
                update_task_status(&mut db, task_index - 1, status);
            }
            'x' => break,
            _ => println!("We don't recognize your input, try again."),
        }
    }
}
