#[derive(Clone, Debug, PartialEq)]
enum Val {
    Name(String),
    Desc(String),
    Status(bool),
}

fn create_element(name: String, desc: String, status: bool) -> Vec<Val> {
    let mut element: Vec<Val> = Vec::new();
    element.push(Val::Name(name));
    element.push(Val::Desc(desc));
    element.push(Val::Status(status));
    element
}

fn set_status(element: Vec<Val>, status: bool) -> Vec<Val> {
    let mut new_element: Vec<Val> = Vec::new();
    for entry in element {
        match entry {
            Val::Name(name) => new_element.push(Val::Name(name.clone())),
            Val::Desc(desc) => new_element.push(Val::Desc(desc.clone())),
            Val::Status(_) => (), // Skip the existing status
        }
    }
    new_element.push(Val::Status(status));
    new_element
}

fn print_element(element: &Vec<Val>) {
    for entry in element {
        match entry {
            Val::Name(name) => println!("  Name: {}", name),
            Val::Desc(desc) => println!("  Description: {}", desc),
            Val::Status(status) => println!("  Status: {}", status),
        }
    }
}

fn print_db(db: &Vec<Vec<Val>>) {
    for element in db {
        print_element(element);
    }
}

fn update_task_status(db: &mut Vec<Vec<Val>>, task: &Vec<Val>, status: bool) {
    if let Some(task_index) = db.iter().position(|t| t == task) {
        let updated_task = set_status(task.clone(), status);
        db[task_index] = updated_task;
    }
}

fn main() {
    println!("Welcome to your command line ToDo manager!\n");

    let mut db: Vec<Vec<Val>> = Vec::new();

    let task1 = create_element(
        String::from("Brush teeth"),
        String::from("I need to brush teeth"),
        false,
    );
    db.push(task1.clone());

    let task2 = create_element(
        String::from("Brush hair"),
        String::from("I need to brush my hair"),
        true,
    );
    db.push(task2.clone());

    // Update the status of task1
    update_task_status(&mut db, &task1, true);

    // Print the updated database
    print_db(&db);
}
