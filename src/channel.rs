use serde::Deserialize;

use crate::servers::Server;

#[derive(Debug, Deserialize)]
pub struct Channel {
    pub id: String,
    pub name: Option<String>,
    #[serde(rename(deserialize = "type"))]
    pub channel_type: u8,
    pub recipients: Option<Vec<String>>,
    pub guild: Option<Server>,
    pub messages: Vec<Message>,
}

#[derive(Debug, Deserialize)]
pub struct Message {
    #[serde(rename(deserialize = "ID"))]
    pub id: String,
    #[serde(rename(deserialize = "Timestamp"))]
    pub timestamp: String,
    #[serde(rename(deserialize = "Contents"))]
    pub contents: Option<String>,
    #[serde(rename(deserialize = "Attachments"))]
    pub attachments: Option<String>,
}
