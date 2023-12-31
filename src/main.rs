use anyhow::Result;
use rusqlite::{Connection, OpenFlags};
use serde::{Deserialize, Serialize};
use serde_json;
use std::env;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize, Debug)]
struct SignalConfig {
    key: String,
}

fn main() {
    let signal_location = get_signal_location().unwrap();
    let db_location = get_db_location(&signal_location).unwrap();

    let key = get_db_key(&signal_location).unwrap();

    println!("{}", db_location);
    let conn = open_db(&db_location, key);

    let mut stmt = conn
        .prepare("select count(id) from messages where hasAttachments=1 order by id")
        .unwrap();
    let rows = stmt.query_map([], |row| Ok(row.get::<_, i64>(0)?)).unwrap();

    for row in rows {
        println!("Number of messages with attachments: {}", row.unwrap());
    }
}

fn open_db(db_location: &String, key: String) -> Connection {
    let flags = OpenFlags::SQLITE_OPEN_READ_ONLY;
    let connection = Connection::open_with_flags(db_location.clone(), flags).unwrap();
    connection
        .pragma_update(None, "key", format!("x'{}'", key))
        .unwrap();
    connection
}

fn get_db_key(signal_location: &String) -> Result<String> {
    let signal_config_file = Path::new(signal_location)
        .join("config.json")
        .canonicalize()?;

    let mut file = File::open(signal_config_file)?;

    // Read the contents of the file into a string.
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let data: SignalConfig = serde_json::from_str(&contents)?;

    Ok(data.key)
}

fn get_db_location(signal_location: &String) -> Result<String, String> {
    let db_path = PathBuf::from(signal_location).join("sql/db.sqlite");

    db_path.to_str().map(|x| x.to_owned()).ok_or(format!(
        "Path is no a valid utf8 string: {}",
        db_path.display()
    ))
}

fn get_signal_location() -> Result<String, String> {
    let os = env::consts::OS;
    let signal_path = PathBuf::from(get_user_profile_folder()?)
        .join(match os {
            "windows" => "AppData/Roaming",
            "linux" => ".config",
            "macos" => "Library/Application Support",
            _ => return Err(format!("Unsupported OS: {}", os)),
        })
        .join("Signal/");

    Ok(signal_path
        .to_str()
        .ok_or(format!(
            "Path is no a valid utf8 string: {}",
            signal_path.display()
        ))?
        .to_owned())
}

fn get_user_profile_folder() -> Result<String, String> {
    let os = env::consts::OS;
    match os {
        "windows" => read_env("USERPROFILE"),
        "macos" | "linux" => read_env("HOME"),
        _ => Err(format!("Unsupported os: {}", os)),
    }
}

fn read_env(var: &str) -> Result<String, String> {
    env::var(var).map_err(|e| format!("Error while reading {}: {}", var, e))
}