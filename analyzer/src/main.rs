use std::{collections::HashMap, env, error::Error};

use rusqlite::Connection;

fn main() -> Result<(), Box<dyn Error>> {
    let db_path = env::args().nth(1).expect("missing db path");
    let db_connection = Connection::open(db_path)?;
    let mut stmt = db_connection.prepare(
        r"select contents from message where contents is not null and contents not like '<%>';",
    )?;
    let messages: Result<Vec<String>, rusqlite::Error> = stmt
        .query_map((), |row| row.get::<usize, String>(0))?
        .collect();

    let mut word_count = HashMap::new();

    messages?.iter().for_each(|message| {
        let splitted_message: Vec<&str> = message.split_ascii_whitespace().collect();
        splitted_message.iter().for_each(|message| {
            *word_count
                .entry(message.to_string().to_lowercase())
                .or_insert(0) += 1
        });
    });

    let mut word_count = word_count.into_iter().collect::<Vec<(String, i32)>>();

    word_count.sort_by(|a, b| b.1.cmp(&a.1));

    println!("{:#?}", word_count);

    Ok(())
}
