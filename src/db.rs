use std::error::Error;

use rusqlite::Connection;

use crate::{account::Account, migration::drop_create_tables, parser::ParserResult};

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
    pub fn save(&self, data: ParserResult) -> OpResult {
        self.drop_create_tables()?;
        self.save_account(data.account)?;
        Ok(())
    }

    fn drop_create_tables(&self) -> OpResult {
        for migration in drop_create_tables() {
            self.db_connection.execute(&migration, [])?;
        }
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
                    account_id, relation_type, nickname, username, avatar, avatar_decoration, discriminator, public_flags
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
}
