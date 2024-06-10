use std::ops::{Add, AddAssign};

#[derive(Clone, Debug)]
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

impl User {
    fn new(name: String, passwd: String, money: i32) -> User {
        let mut user = User(Vec::new());
        user.0.push(Data::Name(name));
        user.0.push(Data::Passwd(passwd));
        user.0.push(Data::Money(money));
        return user;
    }

    fn add_money(&mut self, add_money: i32) {
        for entry in &mut self.0 {
            if let Data::Money(money) = entry {
                *money += add_money;
            }
        }
    }

    fn print_user(&self) {
        for entry in &self.0 {
            match entry {
                Data::Name(name) => println!("Name: {}", name),
                Data::Passwd(_) => (),
                Data::Money(money) => println!("Money: {}", money.to_string())
            }
        }
    }
}

fn main() {
    println!("Welcome to GooseBank!");

    let mut user: User = User::new("Goulash".to_string(), "123".to_string(), 500);
    user.print_user();
    user.add_money(200);
    user.print_user();
}