use std::{collections::HashMap, env, fs::File, io::BufReader};

use account::Account;
use channel::{Channel, Message};
use serde::Deserialize;
use servers::Server;

mod account;
mod channel;
mod servers;

fn main() {
    let dis_data_path = env::args().nth(1).expect("should be at least one argument");
    let user = read_user(&dis_data_path).unwrap();
    println!("{:#?}", user);
    let servers = read_servers(&dis_data_path).unwrap();
    println!("{:#?}", servers);
    let messages = read_messages(&dis_data_path).unwrap();
    println!("{:#?}", messages);
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
        .map(|(key, _val)| -> serde_json::Result<Server> {
            let server_file =
                File::open(format!("{}/servers/{}/guild.json", dis_data_path, key)).unwrap();
            let server_reader = BufReader::new(server_file);
            let result: serde_json::Result<Server> = serde_json::from_reader(server_reader);
            result
        })
        .collect();

    servers
}

fn read_messages(dis_data_path: &str) -> Result<Vec<Channel>, String> {
    let channels_index = File::open(format!("{}/messages/index.json", dis_data_path)).unwrap();
    let index_reader = BufReader::new(channels_index);
    let index: HashMap<String, Option<String>> =
        serde_json::from_reader(index_reader).map_err(|err| err.to_string())?;

    let channels: Result<Vec<Channel>, String> = index
        .iter()
        .map(|(key, val)| -> Result<Channel, String> {
            let channel_path = format!("{}/messages/c{}/", dis_data_path, key);
            let channel_file = File::open(format!("{}/channel.json", channel_path)).unwrap();
            let channel_reader = BufReader::new(channel_file);
            let channel_info: ChannelInfo =
                serde_json::from_reader(channel_reader).map_err(|err| err.to_string())?;
            let messages_file = File::open(format!("{}/messages.csv", channel_path)).unwrap();
            let message_reader = BufReader::new(messages_file);
            let mut rdr = csv::Reader::from_reader(message_reader);
            let messages: Result<Vec<Message>, String> = rdr
                .deserialize()
                .map(|result| result.map_err(|err| err.to_string()))
                .collect();

            let messages = messages?;

            Ok(Channel {
                id: channel_info.id,
                name: val.to_owned(),
                channel_type: channel_info.channel_type,
                recipients: channel_info.recipients,
                guild: channel_info.guild,
                messages,
            })
        })
        .collect();

    channels
}

#[derive(Debug, Deserialize)]
struct ChannelInfo {
    id: String,
    #[serde(rename(deserialize = "type"))]
    channel_type: u8,
    recipients: Option<Vec<String>>,
    guild: Option<Server>,
}
