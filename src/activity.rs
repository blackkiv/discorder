use std::collections::HashMap;

use serde::Deserialize;
use serde_json::Value;

/// Activity info
///
/// contains:
/// - common activity info
/// - other info
#[derive(Debug, Deserialize)]
pub struct Activity {
    pub event_type: String,
    pub event_id: String,
    pub user_id: String,
    pub domain: String,
    pub accepted_languages: Vec<String>,
    pub accepted_languages_weighted: Vec<String>,
    pub client_send_timestamp: String,
    pub client_track_timestamp: String,
    pub timestamp: String,
    #[serde(flatten)]
    pub other: HashMap<String, Value>,
}
