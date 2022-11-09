use std::{env, fs::File, io::BufReader};

use user::User;

mod user;

fn main() {
    let dis_data_path = env::args().nth(1).expect("should be at least one argument");
    let user = read_user(dis_data_path).unwrap();
    println!("{:#?}", user)
}

fn read_user(dis_data_path: String) -> serde_json::Result<User> {
    let user_file = File::open(format!("{}/account/user.json", dis_data_path)).unwrap();
    let user_reader = BufReader::new(user_file);
    serde_json::from_reader(user_reader)
}
