use std::{collections::HashMap, env, fs::File, io::BufReader};

use account::Account;
use servers::Server;

mod account;
mod servers;

fn main() {
    let dis_data_path = env::args().nth(1).expect("should be at least one argument");
    let user = read_user(&dis_data_path).unwrap();
    println!("{:#?}", user);
    let servers = read_servers(&dis_data_path).unwrap();
    println!("{:#?}", servers);
}

fn read_user(dis_data_path: &str) -> serde_json::Result<Account> {
    let user_file = File::open(format!("{}/account/user.json", dis_data_path)).unwrap();
    let user_reader = BufReader::new(user_file);
    serde_json::from_reader(user_reader)
}

fn read_servers(dis_data_path: &str) -> serde_json::Result<Vec<Server>> {
    let servers_index = File::open(format!("{}/servers/index.json", dis_data_path)).unwrap();
    let index_reader = BufReader::new(servers_index);
    let index: HashMap<String, String> = serde_json::from_reader(index_reader)?;

    let servers: serde_json::Result<Vec<Server>> = index
        .iter()
        .map(|(key, _val)| {
            let server_file =
                File::open(format!("{}/servers/{}/guild.json", dis_data_path, key)).unwrap();
            let server_reader = BufReader::new(server_file);
            let result: serde_json::Result<Server> = serde_json::from_reader(server_reader);
            result
        })
        .collect();

    servers
}
