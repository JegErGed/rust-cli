mod data_struct;
use data_struct::{User, DB};
use std::env;
use std::path::{Path, PathBuf};
use std::{fs, io::Write, string};
fn main() {
    println!("Welcome to GooseBank!");
    let mut db: DB = DB::new();
    let exe_path: PathBuf = env::current_exe().expect("Failed to get current exe path");
    let path = exe_path
        .parent()
        .expect("Failed to get parent directory")
        .join("gooseBankDB.json.db");
    let path = path
        .to_str()
        .expect("Failed to convert path to string")
        .to_string();
    if Path::new(&path).exists() {
        db = DB::load_from_file(&path)
    }
    /*
    db.add_user("Goulash".to_string(), "123".to_string(), 550);
    db.add_user("Johan".to_string(), "234".to_string(), 10000000000);
     */

    let serialized_db: String = db.serialize_db();
    println!("{}", serialized_db);
    let mut f = fs::File::create("gooseBankDB.json").expect("Failed to create DB");
    f.write_all(serialized_db.as_bytes())
        .expect("Failed to save to DB");
}
