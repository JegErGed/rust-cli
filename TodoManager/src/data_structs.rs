use regex::Regex;
use std::fs::File;
use std::io::Write;
use std::io::{self, Read};

#[derive(Clone, Debug, PartialEq)]
pub enum Val {
    Name(String),
    Desc(String),
    Status(bool),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Elm(pub Vec<Val>);

impl Elm {
    pub fn new(name: String, desc: String, status: bool) -> Elm {
        let mut element = Elm(Vec::new());
        element.0.push(Val::Name(name));
        element.0.push(Val::Desc(desc));
        element.0.push(Val::Status(status));
        return element;
    }

    pub fn set_status(&mut self, status: bool) {
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

    pub fn print_element(self: &Elm) {
        for entry in &self.0 {
            match entry {
                Val::Name(name) => println!("  Name: {}", name),
                Val::Desc(desc) => println!("  Description: {}", desc),
                Val::Status(status) => {
                    let status_str = if *status {
                        "Completed!"
                    } else {
                        "Not completed!"
                    };
                    println!("  Status: {}", status_str);
                }
            }
        }
    }
    pub fn element_to_text(self: &Elm) -> String {
        let mut data: String = String::new();
        for entry in &self.0 {
            match entry {
                Val::Name(name) => data += &format!("_name_{name}"),
                Val::Desc(desc) => data += &format!("_desc_{desc}"),
                Val::Status(status) => data += &format!("_stat_{status}"),
            }
        }
        return data;
    }
    pub fn is_completed(&self) -> bool {
        for entry in &self.0 {
            if let Val::Status(status) = entry {
                return *status;
            }
        }
        false
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct DB(pub Vec<Elm>);
impl DB {
    pub fn new() -> DB {
        DB(Vec::new())
    }
    pub fn remove_task(&mut self, index: usize) {
        if index < self.0.len() {
            self.0.remove(index);
        } else {
            println!("Invalid task index.");
        }
    }
    pub fn print_db(&mut self) {
        for (index, element) in self.0.iter().enumerate() {
            println!("Task {}:", index + 1);
            element.print_element();
        }
    }
    pub fn update_task_status(&mut self, task_index: usize, status: bool) {
        self.0[task_index].set_status(status);
    }
    pub fn add_create_element(&mut self, name: String, desc: String, status: bool) {
        self.0.push(Elm::new(name, desc, status));
    }
    pub fn user_add_create_element(&mut self) {
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
        self.add_create_element(name, desc, false)
    }
    pub fn remove_completed_tasks(&mut self) {
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
    pub fn save_to_file(&self, path: String) {
        pub fn save_string_to_file(filename: &str, content: &str) -> io::Result<()> {
            // Create or open the file and handle the Result
            let mut file = File::create(filename)?;

            // Write the string content to the file
            file.write_all(content.as_bytes())?;

            Ok(())
        }
        let mut data: String = String::new();
        for entry in &self.0 {
            data += &entry.element_to_text()
        }

        // Attempt to save the string to a file
        match save_string_to_file(&path, &data) {
            Ok(_) => println!("String saved to file successfully."),
            Err(e) => eprintln!("Failed to save string to file: {}", e),
        }
    }
    pub fn reload_from_file(path: &String) -> DB {
        let mut db = DB::new();
        pub fn read_string_from_file(filename: &str) -> io::Result<String> {
            let mut file = File::open(filename)?;
            let mut content = String::new();
            file.read_to_string(&mut content)?;
            Ok(content)
        }
        pub fn str_to_bool(s: &str) -> bool {
            match s {
                "true" => true,
                "false" => false,
                _ => false, // Default to false if input is invalid
            }
        }
        let content = match read_string_from_file(&path) {
            Ok(content) => {
                content
            }
            Err(e) => {
                eprintln!("Failed to read file: {}", e);
                return db;
            }
        };

        // Create a regex pattern to match the structure
        let re = Regex::new(r"_name_(.*?)_desc_(.*?)_stat_(true|false)").unwrap();

        // Iterate over all matches
        for caps in re.captures_iter(&content) {
            let name = caps
                .get(1)
                .or(caps.get(4))
                .map_or("", |m| m.as_str())
                .to_string();
            let desc = caps
                .get(2)
                .or(caps.get(5))
                .map_or("", |m| m.as_str())
                .to_string();
            let stat = caps
                .get(3)
                .or(caps.get(6))
                .map_or("", |m| m.as_str())
                .to_string();

            // Call the add_create_element method on the DB instance
            db.add_create_element(name, desc, str_to_bool(&stat));
        }
        return db;
    }
}
