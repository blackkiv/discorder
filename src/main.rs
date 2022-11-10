use std::{
    collections::{HashMap, HashSet},
    env,
    fs::{self, File},
    io::{BufRead, BufReader},
    time::Instant,
};

use account::Account;
use activity::Activity;
use channel::{Channel, Message};
use serde::Deserialize;
use servers::Server;

mod account;
mod activity;
mod channel;
mod servers;

fn main() {
    let now = Instant::now();
    let dis_data_path = env::args().nth(1).expect("should be at least one argument");
    let user = read_user(&dis_data_path).unwrap();
    println!("{:#?}", user);
    let servers = read_servers(&dis_data_path).unwrap();
    println!("servers {}", servers.len());
    let messages = read_messages(&dis_data_path).unwrap();
    println!("messages {}", messages.len());
    let activities = read_activities(&dis_data_path, ActivityType::Analytics).unwrap();
    let event_types: HashSet<String> = activities
        .iter()
        .map(|activity| activity.event_type.to_string())
        .collect();
    println!("{}|{}", activities.len(), event_types.len());
    let activities = read_activities(&dis_data_path, ActivityType::Modeling).unwrap();
    let event_types: HashSet<String> = activities
        .iter()
        .map(|activity| activity.event_type.to_string())
        .collect();
    println!("{}|{}", activities.len(), event_types.len());
    let activities = read_activities(&dis_data_path, ActivityType::Reporting).unwrap();
    let event_types: HashSet<String> = activities
        .iter()
        .map(|activity| activity.event_type.to_string())
        .collect();
    println!("{}|{}", activities.len(), event_types.len());
    let activities = read_activities(&dis_data_path, ActivityType::Tns).unwrap();
    let event_types: HashSet<String> = activities
        .iter()
        .map(|activity| activity.event_type.to_string())
        .collect();
    println!("{}|{}", activities.len(), event_types.len());
    let elapsed = now.elapsed();
    println!("Elapsed {:.2?}", elapsed);
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

fn read_activities(
    dis_data_path: &str,
    activity_type: ActivityType,
) -> Result<Vec<Activity>, String> {
    let mut activity_dir = fs::read_dir(format!(
        "{}/activity/{}",
        dis_data_path,
        format!("{:?}", activity_type).to_lowercase()
    ))
    .map_err(|err| err.to_string())?;
    let activity_entry = activity_dir
        .next()
        .ok_or("activity not found")?
        .map_err(|err| err.to_string())?;
    let activity_path = activity_entry
        .path()
        .as_os_str()
        .to_string_lossy()
        .to_string();
    let activity_file = File::open(activity_path).map_err(|err| err.to_string())?;
    let mut activity_reader = BufReader::new(activity_file);
    let mut buf = String::new();
    let mut activities: Vec<Activity> = Vec::new();
    while let Ok(buf_len) = activity_reader.read_line(&mut buf) {
        if buf_len == 0 {
            break;
        }
        activities.push(serde_json::from_str(&buf).map_err(|err| err.to_string())?);
        buf.clear();
    }
    Ok(activities)
}

#[derive(Debug)]
enum ActivityType {
    Analytics,
    Modeling,
    Reporting,
    Tns,
}
