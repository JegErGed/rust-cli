mod data_struct;
use data_struct::{Data, User};
fn main() {
    println!("Welcome to GooseBank!");

    let mut user = User::new("Goulash".to_string(), "123".to_string(), 500);
    let sdata = serde_json::to_string(&user);

    if sdata.is_err() {
        println!("Error, filed to Serialize structure {}", sdata.unwrap_err());
        std::process::exit(1);
    }

    let sdata = sdata.unwrap();

    println!("Serialized data: {}", sdata);

    let mut f = fs::File::create("gooseBankDB.json").expect("Failed to create DB");
    f.write_all(sdata.as_bytes()).expect("Failed to save to DB");

    user.print_user("123");
    user.update_money(200, "123");
    user.print_user("123");
}