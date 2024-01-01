use crate::message_content::MessageContent;
use anyhow::{anyhow, Result};
//use args::Args;
use clap::Parser;
use mime_guess;
use rusqlite::{Connection, OpenFlags};
use serde::Serialize;
use serde_json;
use signal_config::SignalConfig;
use std::env;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

mod args;
mod message_content;
mod signal_config;

fn main() -> Result<()> {
    //let args = Args::parse();

    let conn = open_connection_to_signal_db()?;

    let mut count_msg_with_attachment_stmt =
        conn.prepare("select count(id) from messages where hasAttachments=1 order by id")?;
    let msg_count =
        count_msg_with_attachment_stmt.query_row([], |row| Ok(row.get::<_, i64>(0)?))?;

    println!("Number of messages with attachments: {}", msg_count);

    let mut stmt =
        conn.prepare("SELECT json FROM messages WHERE hasAttachments = 1 ORDER BY id")?;

    // Map the rows into a desired structure.
    let message_iter = stmt.query_map([], |row| {
        Ok(row.get::<_, String>(0)?)
    })?;

    // Iterate over the returned rows.
    for message in message_iter {
        let json = message?;
        //println!("ID: {}, JSON: {}", id, json);

        let message: Result<MessageContent, serde_json::Error> = serde_json::from_str(&json);
        // Handle the result
        match message {
            Err(e) => {
                eprintln!("Error parsing JSON: {}:\n {}", e, json);
            },
            _ => {}
        }
    }

    Ok(())
}

fn open_connection_to_signal_db() -> Result<Connection> {
    let signal_location = get_signal_location()?;

    let db_location = get_db_location(&signal_location)?;

    let key = get_db_key(&signal_location)?;

    let conn = open_db(&db_location, key)?;

    Ok(conn)
}

fn open_db(db_location: &String, key: String) -> Result<Connection> {
    let flags = OpenFlags::SQLITE_OPEN_READ_ONLY;
    let connection = Connection::open_with_flags(db_location.clone(), flags)?;
    connection.pragma_update(None, "key", format!("x'{}'", key))?;

    Ok(connection)
}

fn get_db_key(signal_location: &String) -> Result<String> {
    let signal_config_file = Path::new(signal_location)
        .join("config.json")
        .canonicalize()?;

    let mut file = File::open(signal_config_file)?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let data: SignalConfig = serde_json::from_str(&contents)?;

    Ok(data.key)
}

fn get_db_location(signal_location: &String) -> Result<String> {
    let db_path = PathBuf::from(signal_location).join("sql/db.sqlite");

    db_path
        .to_str()
        .map(|x| x.to_owned())
        .ok_or(anyhow!(format!(
            "Path is no a valid utf8 string: {}",
            db_path.display()
        )))
}

fn get_signal_location() -> Result<String> {
    let os = env::consts::OS;
    let signal_path = PathBuf::from(get_user_profile_folder()?)
        .join(match os {
            "windows" => "AppData/Roaming",
            "linux" => ".config",
            "macos" => "Library/Application Support",
            _ => return Err(anyhow!("Unsupported OS: {}", os)),
        })
        .join("Signal/");

    Ok(signal_path
        .to_str()
        .ok_or(anyhow!(
            "Path is no a valid utf8 string: {}",
            signal_path.display()
        ))?
        .to_owned())
}

fn get_user_profile_folder() -> Result<String> {
    let os = env::consts::OS;
    match os {
        "windows" => Ok(read_env("USERPROFILE")?),
        "macos" | "linux" => Ok(read_env("HOME")?),
        _ => Err(anyhow!("Unsupported os: {}", os)),
    }
}

fn read_env(var: &str) -> Result<String> {
    env::var(var).map_err(|e| anyhow!("Error while reading environment variable {}: {}", var, e))
}
