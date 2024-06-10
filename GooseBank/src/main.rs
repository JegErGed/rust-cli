use std::{
    hash::{DefaultHasher, Hash},
    ops::{Add, AddAssign, Sub, SubAssign},
};

#[derive(Clone, Debug, Hash)]
enum Data {
    Name(String),
    Passwd(String),
    Money(i32),
}

struct User(Vec<Data>);

impl Add<i32> for User {
    type Output = User;

    fn add(self, other: i32) -> User {
        let mut new_entries = Vec::new();

        for entry in self.0 {
            match entry {
                Data::Name(name) => new_entries.push(Data::Name(name)),
                Data::Passwd(passwd) => new_entries.push(Data::Passwd(passwd)),
                Data::Money(money) => new_entries.push(Data::Money(money + other)),
            }
        }

        User(new_entries)
    }
}

impl AddAssign<i32> for User {
    fn add_assign(&mut self, other: i32) {
        for entry in &mut self.0 {
            if let Data::Money(money) = entry {
                *money += other;
            }
        }
    }
}

impl Sub<i32> for User {
    type Output = User;

    fn sub(self, other: i32) -> User {
        let mut new_entries = Vec::new();

        for entry in self.0 {
            match entry {
                Data::Name(name) => new_entries.push(Data::Name(name)),
                Data::Passwd(passwd) => new_entries.push(Data::Passwd(passwd)),
                Data::Money(money) => new_entries.push(Data::Money(money - other)),
            }
        }

        User(new_entries)
    }
}

impl SubAssign<i32> for User {
    fn sub_assign(&mut self, other: i32) {
        for entry in &mut self.0 {
            if let Data::Money(money) = entry {
                *money -= other;
            }
        }
    }
}

impl User {
    fn hash(passwd: String, salt: &str) -> String {
        let mut hasher = DefaultHasher::new();
        let mut output: String = String::new();
        output += &salt;
        output += &passwd;
        output.hash(&mut hasher);
        return output;
    }

    fn new(name: String, passwd: String, money: i32, salt: &str) -> User {
        let mut user = User(Vec::new());
        user.0.push(Data::Name(name));
        user.0.push(Data::Passwd(User::hash(passwd, &salt)));
        user.0.push(Data::Money(money));
        return user;
    }

    fn passwd(&self, input_pass: &str, salt: &str) -> bool {
        for entry in &self.0 {
            if let Data::Passwd(passwd) = entry {
                return passwd.to_string() == User::hash(input_pass.to_string(), salt);
            }
        }
        return false;
    }

    fn update_money(&mut self, add_money: i32, input_pass: &str, salt: &str) {
        if !&self.passwd(input_pass, &salt) {
            println!("Access not granted!");
            return;
        }
        for entry in &mut self.0 {
            if let Data::Money(money) = entry {
                *money += add_money;
            }
        }
    }

    fn print_user(&self, input_pass: &str, salt: &str) {
        if !&self.passwd(input_pass, &salt) {
            println!("Access not granted!");
            return;
        }
        for entry in &self.0 {
            match entry {
                Data::Name(name) => println!("Name: {}", name),
                Data::Passwd(_) => (),
                Data::Money(money) => println!("Money: {}", money.to_string()),
            }
        }
    }
}

fn main() {
    println!("Welcome to GooseBank!");

    let salt: String = "goulashBanken".to_string();
    let mut user: User = User::new("Goulash".to_string(), "123".to_string(), 500, &salt);
    user.print_user("123", &salt);
    user.update_money(200, "123", &salt);
    user.print_user("123", &salt);
}
