use std::{
    collections::hash_map::DefaultHasher, fs, hash::{Hash, Hasher}, io::Write, ops::{Add, AddAssign, Sub, SubAssign}, string
};
use serde::{Deserialize, Serialize};
use serde_json;


#[derive(Clone, Debug, Hash, Deserialize, Serialize, PartialEq)]
pub enum Data {
    Name(String),
    Passwd(u64),
    Money(i32),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct User(Vec<Data>);

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
    pub fn hash<T>(passwd: T) -> u64
    where
        T: Hash,
    {
        let mut hasher = DefaultHasher::new();
        passwd.hash(&mut hasher);
        hasher.finish()
    }

    pub fn new(name: String, passwd: String, money: i32) -> User {
        let mut user = User(Vec::new());
        user.0.push(Data::Name(name));
        user.0.push(Data::Passwd(User::hash(passwd)));
        user.0.push(Data::Money(money));
        return user;
    }

    pub fn passwd(&self, input_pass: &str) -> bool {
        let hashed_input = User::hash(input_pass);
        for entry in &self.0 {
            if let Data::Passwd(stored_passwd) = entry {
                return *stored_passwd == hashed_input;
            }
        }
        return false;
    }

    pub fn update_money(&mut self, add_money: i32, input_pass: &str) {
        if !self.passwd(input_pass) {
            println!("Access not granted!");
            return;
        }
        for entry in &mut self.0 {
            if let Data::Money(money) = entry {
                *money += add_money;
            }
        }
    }

    pub fn print_user(&self, input_pass: &str) {
        if !self.passwd(input_pass) {
            println!("Access not granted!");
            return;
        }
        for entry in &self.0 {
            match entry {
                Data::Name(name) => println!("Name: {}", name),
                Data::Passwd(_) => (),
                Data::Money(money) => println!("Money: {}", money),
            }
        }
    }
}


#[derive(Clone, Debug, PartialEq)]
pub struct DB(pub Vec<User>);
impl DB {
    pub fn new() -> DB {
        DB(Vec::new())
    }

    pub fn add_user(&mut self, name: String, passwd: String, money: i32) {
        self.0.push(User::new(name, passwd, money));
    }
}