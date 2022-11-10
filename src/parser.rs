use std::{
    collections::HashMap,
    error::Error,
    fs::{self, File},
    io::{BufRead, BufReader},
};

use serde::Deserialize;

use crate::{
    account::Account,
    activity::{Activity, ActivityType},
    channel::{Channel, Message},
    servers::Server,
};

type BoxErrorResult<T> = Result<T, Box<dyn Error>>;

pub struct ParserResult {
    pub account: Account,
    // pub activities: HashMap<ActivityType, Vec<Activity>>,
    // pub channels: Vec<Channel>,
    // pub servers: Vec<Server>,
}

pub struct Parser {
    discord_data_path: String,
}

impl Parser {
    pub fn new(discord_data_path: &str) -> Self {
        Parser {
            discord_data_path: discord_data_path.to_string(),
        }
    }
}

impl Parser {
    pub fn parse(&self) -> Result<ParserResult, Box<dyn Error>> {
        let account = self.read_account()?;
        // let servers = self.read_servers()?;
        // let channels = self.read_channels()?;
        // let activities = self.read_all_activities()?;
        Ok(ParserResult {
            account,
            // activities,
            // channels,
            // servers,
        })
    }

    fn read_account(&self) -> BoxErrorResult<Account> {
        let account_reader = BufReader::new(File::open(format!(
            "{}/account/user.json",
            self.discord_data_path
        ))?);
        let account: Account = serde_json::from_reader(account_reader)?;
        Ok(account)
    }

    fn read_servers(&self) -> BoxErrorResult<Vec<Server>> {
        let servers_index =
            File::open(format!("{}/servers/index.json", self.discord_data_path)).unwrap();
        let index_reader = BufReader::new(servers_index);
        let index: HashMap<String, String> = serde_json::from_reader(index_reader)?;

        index
            .iter()
            .map(|(key, _val)| -> BoxErrorResult<Server> {
                let server_reader = BufReader::new(File::open(format!(
                    "{}/servers/{}/guild.json",
                    self.discord_data_path, key
                ))?);
                let result: Server = serde_json::from_reader(server_reader)?;
                Ok(result)
            })
            .collect::<BoxErrorResult<Vec<Server>>>()
    }

    fn read_channels(&self) -> BoxErrorResult<Vec<Channel>> {
        let index_reader = BufReader::new(File::open(format!(
            "{}/messages/index.json",
            self.discord_data_path
        ))?);
        let index: HashMap<String, Option<String>> =
            serde_json::from_reader(index_reader).map_err(|err| err.to_string())?;

        index
            .iter()
            .map(|(key, val)| -> BoxErrorResult<Channel> {
                let channel_path = format!("{}/messages/c{}/", self.discord_data_path, key);
                let channel_reader =
                    BufReader::new(File::open(format!("{}/channel.json", channel_path))?);
                let channel_info: ChannelInfo = serde_json::from_reader(channel_reader)?;
                let message_reader =
                    BufReader::new(File::open(format!("{}/messages.csv", channel_path))?);
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
            .collect::<BoxErrorResult<Vec<Channel>>>()
    }

    fn read_all_activities(&self) -> BoxErrorResult<HashMap<ActivityType, Vec<Activity>>> {
        let mut activities = HashMap::new();
        activities.insert(
            ActivityType::Analytics,
            self.read_activities(ActivityType::Analytics)?,
        );
        activities.insert(
            ActivityType::Modeling,
            self.read_activities(ActivityType::Modeling)?,
        );
        activities.insert(
            ActivityType::Reporting,
            self.read_activities(ActivityType::Reporting)?,
        );
        activities.insert(ActivityType::Tns, self.read_activities(ActivityType::Tns)?);

        Ok(activities)
    }

    fn read_activities(&self, activity_type: ActivityType) -> BoxErrorResult<Vec<Activity>> {
        let mut activity_dir = fs::read_dir(format!(
            "{}/activity/{}",
            self.discord_data_path,
            format!("{:?}", activity_type).to_lowercase()
        ))
        .map_err(|err| err.to_string())?;
        let activity_entry = activity_dir.next().ok_or("activity not found")??;
        let activity_path = activity_entry.path().to_string_lossy().to_string();
        let mut activity_reader = BufReader::new(File::open(activity_path)?);
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
}

#[derive(Debug, Deserialize)]
struct ChannelInfo {
    id: String,
    #[serde(rename(deserialize = "type"))]
    channel_type: u8,
    recipients: Option<Vec<String>>,
    guild: Option<Server>,
}
