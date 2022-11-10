use std::error::Error;

use sqlite::Connection;

use crate::{migration::drop_create_tables, parser::ParserResult};

type OpResult = Result<(), Box<dyn Error>>;

pub struct Dao {
    db_connection: Connection,
}

impl Dao {
    pub fn new(db_name: &str) -> sqlite::Result<Self> {
        let connection = sqlite::open(db_name)?;
        Ok(Dao {
            db_connection: connection,
        })
    }
}

impl Dao {
    pub fn save(&self, data: ParserResult) -> OpResult {
        self.drop_create_tables()?;
        Ok(())
    }

    fn drop_create_tables(&self) -> OpResult {
        self.db_connection.execute(drop_create_tables())?;
        Ok(())
    }
}
