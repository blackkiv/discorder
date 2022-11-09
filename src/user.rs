use serde::Deserialize;

// User info
// 
// contains:
// - basic user info
// - user_profile_metadata
// - relationships
#[derive(Debug, Deserialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub discriminator: i32,
    pub email: String,
    pub verified: bool,
    pub avatar_hash: String,
    pub has_mobile: bool,
    pub needs_email_verification: bool,
    pub premium_until: Option<String>,
    pub flags: i64,
    pub phone: Option<String>,
    pub temp_banned_until: Option<String>,
    pub ip: String,
    pub user_profile_metadata: UserMetadata,
    pub relationships: Vec<Relationship>,
}

#[derive(Debug, Deserialize)]
pub struct UserMetadata {
    pub user_id: String,
    pub boosting_started_at: Option<String>,
    pub premium_started_at: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Relationship {
    pub id: String,
    #[serde(rename(deserialize = "type"))]
    pub relation_type: u32,
    pub nickname: Option<String>,
    pub user: RelationUser,
}

#[derive(Debug, Deserialize)]
pub struct RelationUser {
    pub id: String,
    pub username: String,
    pub avatar: Option<String>,
    pub avatar_decoration: Option<String>,
    pub discriminator: String,
    pub public_flags: u32,
}
