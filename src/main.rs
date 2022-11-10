use std::{env, error::Error, time::Instant};

use crate::{db::Dao, parser::Parser};

mod account;
mod activity;
mod channel;
mod db;
mod migration;
mod parser;
mod servers;

fn main() -> Result<(), Box<dyn Error>> {
    let dis_data_path = env::args().nth(1).expect("should be at least one argument");

    let now = Instant::now();
    let parser = Parser::new(&dis_data_path);
    let parser_result = parser.parse()?;

    let elapsed = now.elapsed();
    println!("[Parsing] Elapsed {:.2?}", elapsed);

    let dao = Dao::new("data.db")?;

    let now = Instant::now();
    dao.save(parser_result)?;

    let elapsed = now.elapsed();
    println!("[Saving] Elapsed {:.2?}", elapsed);

    Ok(())
}
