mod data_struct;
use data_struct::User;
use std::{fs, io::Write, string};
fn main() {
    println!("Welcome to GooseBank!");

    let mut user = User::new("Goulash".to_string(), "123".to_string(), 550);
    let mut user2 = User::new("Johan".to_string(), "234".to_string(), 10000000000);

    let mut serialized: String = String::new();
    
    serialized += &user.serialize_user();
    serialized += &user2.serialize_user();


    println!("{}", serialized);

    let mut f = fs::File::create("gooseBankDB.json").expect("Failed to create DB");
    f.write_all(serialized.as_bytes()).expect("Failed to save to DB");

    user.print_user("123");
    user.update_money(200, "123");
    user.print_user("123");

    user2.print_user("234");
}
