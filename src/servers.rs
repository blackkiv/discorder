use serde::Deserialize;

/// Server info
/// 
/// contains:
/// - basic server info
#[derive(Debug, Deserialize)]
pub struct Server {
    pub id: String,
    pub name: String,
}
