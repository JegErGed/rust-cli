use serde::{Deserialize, Serialize};
use serde_json;
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    ops::{Add, AddAssign, Sub, SubAssign},
};

#[derive(Clone, Debug, Hash, Deserialize, Serialize, PartialEq)]
pub struct User {
    name: String,
    passwd: u64,
    money: i64,
}

impl Add<i64> for User {
    type Output = User;

    fn add(self, other: i64) -> User {
        User {
            name: self.name,
            passwd: self.passwd,
            money: self.money + other,
        }
    }
}

impl AddAssign<i64> for User {
    fn add_assign(&mut self, other: i64) {
        self.money += other;
    }
}

impl Sub<i64> for User {
    type Output = User;

    fn sub(self, other: i64) -> User {
        User {
            name: self.name,
            passwd: self.passwd,
            money: self.money - other,
        }
    }
}

impl SubAssign<i64> for User {
    fn sub_assign(&mut self, other: i64) {
        self.money -= other;
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

    pub fn new(name: String, passwd: String, money: i64) -> User {
        User {
            name,
            passwd: User::hash(passwd),
            money,
        }
    }

    pub fn passwd(&self, input_pass: &str) -> bool {
        let hashed_input = User::hash(input_pass);
        self.passwd == hashed_input
    }

    pub fn update_money(&mut self, add_money: i64, input_pass: &str) {
        if !self.passwd(input_pass) {
            println!("Access not granted!");
            return;
        }
        self.money += add_money;
    }

    pub fn print_user(&self, input_pass: &str) {
        if !self.passwd(input_pass) {
            println!("Access not granted!");
            return;
        }
        println!("Name: {}", self.name);
        println!("Money: {:.2}", self.money as f64 / 100.0);
    }

    pub fn serialize_user(&self) -> String {
        let sdata = serde_json::to_string(&self);

        if sdata.is_err() {
            println!("Error, failed to serialize structure: {}", sdata.unwrap_err());
            std::process::exit(1);
        }

        sdata.unwrap()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct DB(pub Vec<User>);

impl DB {
    pub fn new() -> DB {
        DB(Vec::new())
    }

    pub fn add_user(&mut self, name: String, passwd: String, money: i64) {
        self.0.push(User::new(name, passwd, money));
    }

    pub fn remove_user(&mut self, index: usize) {
        if index < self.0.len() {
            self.0.remove(index);
        } else {
            println!("Invalid index");
        }
    }

    pub fn serialize_db(&self) -> String{
        let serialized_users: Vec<String> = self.0.iter().map(|user| user.serialize_user()).collect();
        let serialized_str = format!("[{}]", serialized_users.join(","));
        return serialized_str;
    }
}
