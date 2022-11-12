use std::{collections::HashMap, error::Error};

use indicatif::ProgressBar;
use rusqlite::Connection;

use crate::{
    account::Account,
    activity::{Activity, ActivityType},
    channel::Channel,
    migration::drop_create_tables,
    parser::ParserResult,
    servers::Server,
};

type OpResult = Result<(), Box<dyn Error>>;

pub struct Dao {
    db_connection: Connection,
}

impl Dao {
    pub fn new(db_name: &str) -> rusqlite::Result<Self> {
        let connection = Connection::open(db_name)?;
        Ok(Dao {
            db_connection: connection,
        })
    }
}

impl Dao {
    pub fn save(&self, data: ParserResult, prog_bar: &ProgressBar) -> OpResult {
        self.drop_create_tables()?;
        prog_bar.inc(1);
        self.save_account(data.account)?;
        prog_bar.inc(1);
        self.save_servers(data.servers)?;
        prog_bar.inc(1);
        self.save_channels(data.channels)?;
        prog_bar.inc(1);
        self.save_activities(data.activities)?;
        prog_bar.inc(1);
        Ok(())
    }

    fn drop_create_tables(&self) -> OpResult {
        self.db_connection.execute_batch(&drop_create_tables())?;
        Ok(())
    }

    fn save_account(&self, account: Account) -> OpResult {
        self.db_connection.execute(
            "
            insert into account values (
                ?1, ?2, ?3, ?4,
                ?5, ?6, ?7, ?8,
                ?9, ?10, ?11, ?12,
                ?13, ?14, ?15
            );
            ",
            (
                account.id.to_string(),
                account.username,
                account.discriminator,
                account.email,
                account.verified,
                account.avatar_hash,
                account.has_mobile,
                account.needs_email_verification,
                account.premium_until,
                account.flags,
                account.phone,
                account.temp_banned_until,
                account.ip,
                account.user_profile_metadata.boosting_started_at,
                account.user_profile_metadata.premium_started_at,
            ),
        )?;

        for relationship in account.relationships.iter() {
            self.db_connection.execute(
                "
                insert into relationship (
                    account_id, relation_type, nickname, username, 
                    avatar, avatar_decoration, discriminator, public_flags
                ) values (
                    ?1, ?2, ?3, ?4, 
                    ?5, ?6, ?7, ?8
                );
                ",
                (
                    account.id.to_string(),
                    relationship.relation_type,
                    relationship.nickname.to_owned(),
                    relationship.user.username.to_owned(),
                    relationship.user.avatar.to_owned(),
                    relationship.user.avatar_decoration.to_owned(),
                    relationship.user.discriminator.to_owned(),
                    relationship.user.public_flags,
                ),
            )?;
        }
        Ok(())
    }

    fn save_servers(&self, servers: Vec<Server>) -> OpResult {
        let servers_value: Vec<String> = servers
            .iter()
            .map(|server| vec![str_to_sql_str(&server.id), str_to_sql_str(&server.name)].join(", "))
            .map(|server| format!("({})", server))
            .collect();

        let servers_value = servers_value.join(", ");
        let sql = format!("insert into server values {};", servers_value);

        self.db_connection.execute(&sql, ())?;
        Ok(())
    }

    fn save_channels(&self, channels: Vec<Channel>) -> OpResult {
        for channel in channels {
            self.db_connection.execute(
                "
                insert into channel values (?1, ?2, ?3);
                ",
                (
                    channel.id.to_string(),
                    channel.channel_type,
                    channel.guild.map(|guild| guild.id),
                ),
            )?;

            for message in channel.messages {
                self.db_connection.execute(
                    "insert into message values (?1, ?2, ?3, ?4, ?5);",
                    (
                        message.id,
                        &channel.id,
                        message.timestamp,
                        message.contents,
                        message.attachments,
                    ),
                )?;
            }

            if let Some(recipients) = channel.recipients {
                let recipients_value: Vec<String> = recipients
                    .iter()
                    .map(|recipient| {
                        vec![str_to_sql_str(&channel.id), str_to_sql_str(recipient)].join(", ")
                    })
                    .map(|recipient| format!("({})", recipient))
                    .collect();

                let recipients_value = recipients_value.join(", ");
                let sql = format!(
                    "insert into channel_recipient (channel_id, recipient) values {};",
                    recipients_value
                );

                self.db_connection.execute(&sql, ())?;
            }
        }

        Ok(())
    }

    fn save_activities(&self, activities: HashMap<ActivityType, Vec<Activity>>) -> OpResult {
        for (activity_type, activities) in activities {
            let mut activity_values = Vec::new();
            let mut accepted_language_values = Vec::new();
            let mut accepted_language_weighted_values = Vec::new();
            for activity in activities {
                activity_values.push(format!(
                    "({}, {}, {}, {}, {}, {}, {}, {}, {})",
                    str_to_sql_str(&activity.event_id),
                    str_to_sql_str(&activity.event_type),
                    str_to_sql_str(format!("{:?}", activity_type).as_str()),
                    str_to_sql_str(&activity.user_id),
                    str_to_sql_str(&activity.domain),
                    str_to_sql_str(&activity.client_send_timestamp),
                    str_to_sql_str(&activity.client_track_timestamp),
                    str_to_sql_str(&activity.timestamp),
                    str_to_sql_str(serde_json::to_string(&activity.other)?.as_str()),
                ));

                for accepted_language in activity.accepted_languages {
                    accepted_language_values.push(format!(
                        "({}, {})",
                        str_to_sql_str(&activity.event_id),
                        str_to_sql_str(&accepted_language)
                    ));
                }
                for accepted_language_weighted in activity.accepted_languages_weighted {
                    accepted_language_weighted_values.push(format!(
                        "({}, {})",
                        str_to_sql_str(&activity.event_id),
                        str_to_sql_str(&accepted_language_weighted)
                    ));
                }
            }
            self.execute_batch(
                "insert into activity (
                    event_id, event_type, activity_type, user_id, 
                    domain, client_send_timestamp, 
                    client_track_timestamp, timestamp, other
                )",
                activity_values,
            )?;
            self.execute_batch(
                "insert into accepted_languages (event_id, language)",
                accepted_language_values,
            )?;
            self.execute_batch(
                "insert into accepted_languages_weighted (event_id, language)",
                accepted_language_weighted_values,
            )?;
        }

        Ok(())
    }

    fn execute_batch(&self, insert: &str, values: Vec<String>) -> OpResult {
        let values = values.join(",\n");
        let sql = format!("{} values {}", insert, values);
        self.db_connection.execute(&sql, ())?;
        Ok(())
    }
}

fn str_to_sql_str(val: &str) -> String {
    format!("'{}'", val.replace('\'', "''"))
}
