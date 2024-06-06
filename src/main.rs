// a simple db in python syntax would have [[name: str, desc: str, status: bool, ], [], [], []...]


 
enum Val {
    Name(String),
    Desc(String),
    Status(bool),
}

fn create_element(name: String, desc: String, status: bool) -> Vec<Val> {
    let mut db: Vec<Val> = Vec::new();
    db.push(Val::Name(name));
    db.push(Val::Desc(desc));
    db.push(Val::Status(status));
    return db;
}

fn set_status(element: Vec<Val>, status: bool) -> Vec<Val> {
    let mut new_element: Vec<Val> = Vec::new(); 
    for entry in element {
        match entry {
            Val::Name(name) => {
                new_element.push(Val::Name(name.clone())) 
            },
            Val::Desc(desc) => { new_element.push(Val::Desc(desc.clone())) },
            other_entry => {
                ;
            }
        }
    }
    new_element.push(Val::Status(status));
    return new_element;
}

fn print_element(element: Vec<Val>) {
    for entry in element {
        match entry {
            Val::Name(name) => println!("  Name: {}", name),
            Val::Desc(desc) => println!("  Description: {}", desc),
            Val::Status(status) => println!("  Status: {}", status),
        }
    }
}
fn print_db(db: Vec<Vec<Val>>) {
    for element in db {
        print_element(element);
    }
}

fn main() {
    println!("Welcome to your command line ToDo manager!\n");
    let mut db: Vec<Vec<Val>> = Vec::new();
    let mut task: Vec<Val> = create_element(String::from("Brush teeth"), String::from("I need to brush teeth"), false);
    db.push(task);
    let mut task2: Vec<Val> = create_element(String::from("Brush hair"), String::from("I need to brush my hair"), true);
    db.push(task2);

    let mut task = set_status(task, true);
    


    print_db(db);




}
