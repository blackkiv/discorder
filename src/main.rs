use std::{env, error::Error, time::Instant};

use indicatif::ProgressBar;

use crate::{db::Dao, parser::Parser};

mod account;
mod activity;
mod channel;
mod db;
mod migration;
mod parser;
mod servers;

fn main() -> Result<(), Box<dyn Error>> {
    let dis_data_path = env::args().nth(1).expect("missing discord data path");
    let db_path = env::args().nth(2).expect("missing db path");
    let prog_bar = ProgressBar::new(9);

    let now = Instant::now();
    let parser = Parser::new(&dis_data_path);
    let parser_result = parser.parse(&prog_bar)?;
    let parsing_elapsed = now.elapsed();

    let dao = Dao::new(&db_path)?;

    let now = Instant::now();
    dao.save(parser_result, &prog_bar)?;
    let elapsed = now.elapsed();
    
    println!("[Parsing] Elapsed {:.2?}", parsing_elapsed);
    println!("[Saving] Elapsed {:.2?}", elapsed);

    Ok(())
}
