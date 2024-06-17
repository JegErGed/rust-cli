mod data_struct;
use data_struct::{User, DB};
use std::{fs, io::Write, string};
fn main() {
    println!("Welcome to GooseBank!");

    let mut db: DB = DB::new();
    db.add_user("Goulash".to_string(), "123".to_string(), 550);
    db.add_user("Johan".to_string(), "234".to_string(), 10000000000);

    let serialized_db: String = db.serialize_db();

    let mut f = fs::File::create("gooseBankDB.json").expect("Failed to create DB");
    f.write_all(serialized_db.as_bytes()).expect("Failed to save to DB");
}
