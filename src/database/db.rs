use dirs::data_local_dir;
use rusqlite::{params, Connection, OpenFlags, Result};
use serde::{Deserialize, Serialize};
use serde_json;
use std::env;
use std::str::FromStr;
use std::{
    fmt::{Display, Formatter},
    path::PathBuf,
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SavedCommand {
    id: i32,
    command: String,
    curl_json: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SavedKey {
    id: i32,
    key: String,
}

#[derive(Debug)]
pub struct DB {
    pub conn: Connection,
}

impl DB {
    pub fn new() -> Result<Self, rusqlite::Error> {
        let mut _path: PathBuf = PathBuf::new();
        if std::env::var("CUTE_DB_PATH").is_ok() {
            _path = PathBuf::from_str(env::var("CUTE_DB_PATH").unwrap().as_str()).unwrap();
        } else {
            _path = DB::get_default_path();
        }
        if !_path.exists() {
            // If it doesn't exist, create it
            if let Err(err) = std::fs::create_dir_all(&_path) {
                std::fs::File::create(&_path).expect("failed to create database");
                eprintln!("Failed to create CuTE directory: {}", err);
            } else {
                println!("CuTE directory created at {:?}", _path);
            }
        }

        let conn_result = Connection::open_with_flags(
            _path.join("CuTE.db"),
            OpenFlags::SQLITE_OPEN_READ_WRITE
                | OpenFlags::SQLITE_OPEN_CREATE
                | OpenFlags::SQLITE_OPEN_URI
                | OpenFlags::SQLITE_OPEN_NO_MUTEX,
        );

        // We Need To Handle Some Errors Here Related To Opening SQLite3 Database Files
        let conn = match conn_result {
            Ok(connection) => connection,
            Err(e) => {
                println!("CuTE Database Error: {:?}", e);
                return Err(e);
            }
        };

        // Begin a transaction
        conn.execute("BEGIN;", params![])?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS commands (id INTEGER PRIMARY KEY, command TEXT, curl_json JSON);",
            params![],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS keys (id INTEGER PRIMARY KEY, key TEXT);",
            params![],
        )?;

        conn.execute("COMMIT;", params![])?;

        Ok(DB { conn })
    }

    pub fn get_default_path() -> PathBuf {
        let dir = data_local_dir().expect("Failed to get data local directory,\nPlease specify a path at $CONFIG/CuTE/config.toml\nOr with the --db_path={path/to/CuTE.db}");
        dir.join("CuTE")
    }

    pub fn add_command(&self, command: &str, json_str: String) -> Result<(), rusqlite::Error> {
        let mut stmt = self
            .conn
            .prepare("INSERT INTO commands (command, curl_json) VALUES (?1, ?2)")?;
        let _ = stmt.execute(params![command, &json_str])?;
        Ok(())
    }

    pub fn delete_command(&self, id: i32) -> Result<(), rusqlite::Error> {
        let mut stmt = self.conn.prepare("DELETE FROM commands WHERE id = ?")?;
        stmt.execute([id])?;
        Ok(())
    }

    pub fn key_exists(&self, key: &str) -> Result<bool> {
        let mut stmt = self
            .conn
            .prepare("SELECT COUNT(*) FROM keys WHERE key = ?")?;
        let count: i64 = stmt.query_row([&key], |row| row.get(0))?;
        Ok(count > 0)
    }

    pub fn command_exists(&self, command: &str) -> Result<bool, rusqlite::Error> {
        let mut stmt = self
            .conn
            .prepare("SELECT COUNT(*) FROM commands WHERE command = ?")?;
        let count: i64 = stmt.query_row([&command], |row| row.get(0))?;
        Ok(count > 0)
    }

    pub fn get_commands(&self) -> Result<Vec<SavedCommand>> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, command, curl_json FROM commands")?;
        let rows = stmt.query_map(params![], |row| {
            Ok(SavedCommand {
                id: row.get(0)?,
                command: row.get(1)?,
                curl_json: row.get(2)?,
            })
        })?;
        let mut commands = Vec::new();
        rows.for_each(|row| {
            commands.push(row.unwrap());
        });
        Ok(commands)
    }

    pub fn delete_key(&self, id: i32) -> Result<()> {
        let mut stmt = self.conn.prepare("DELETE FROM keys WHERE id = ?")?;
        stmt.execute([id])?;
        Ok(())
    }

    pub fn add_key(&self, key: &str) -> Result<()> {
        if self.key_exists(key).unwrap() {
            return Ok(());
        }
        let mut stmt = self.conn.prepare("INSERT INTO keys (key) VALUES (?1)")?;
        let _ = stmt.execute(params![key])?;
        Ok(())
    }

    pub fn get_keys(&self) -> Result<Vec<SavedKey>> {
        let mut stmt = self.conn.prepare("SELECT id, key FROM keys")?;
        let rows = stmt.query_map(params![], |row| {
            Ok(SavedKey {
                id: row.get(0)?,
                key: row.get(1)?,
            })
        })?;
        let mut keys = Vec::new();
        for key in rows {
            keys.push(key?);
        }
        Ok(keys)
    }
}

impl Display for SavedCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.command)
    }
}

impl SavedCommand {
    // We nned to allow the user to write out the response to a file,
    // so at some point we may need to read it back in
    pub fn to_json(&self) -> Result<String> {
        Ok(serde_json::to_string(&self).expect("Failed to serialize"))
    }

    pub fn get_id(&self) -> i32 {
        self.id
    }

    pub fn from_json(json: &str) -> Result<Self> {
        Ok(serde_json::from_str(json).expect("Failed to deserialize"))
    }

    pub fn get_curl_json(&self) -> &str {
        &self.curl_json
    }

    pub fn get_command(&self) -> &str {
        &self.command
    }
}

impl Display for SavedKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, " ID: {} | Key: {}", self.id, self.key)
    }
}

impl SavedKey {
    pub fn new(key: &str) -> Self {
        SavedKey {
            id: 0,
            key: key.to_string(),
        }
    }

    pub fn get_id(&self) -> i32 {
        self.id
    }
    pub fn get_key(&self) -> &str {
        &self.key
    }

    pub fn is_key(&self, key: &str) -> bool {
        self.key == key
    }

    pub fn to_json(&self) -> Result<String> {
        Ok(serde_json::to_string(&self).expect("Failed to serialize"))
    }

    pub fn from_json(json: &str) -> Result<Self> {
        Ok(serde_json::from_str(json).expect("Failed to deserialize"))
    }
}
