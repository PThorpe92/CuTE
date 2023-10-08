use std::fmt::{Display, Formatter};

use dirs::data_local_dir;
use rusqlite::{params, Connection, OpenFlags, Result};
use serde::{Deserialize, Serialize};
use serde_json;

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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SavedUrl {
    id: i32,
    url: String,
}

#[derive(Debug)]
pub struct DB {
    pub conn: Connection,
}

impl DB {
    pub fn new() -> Result<Self, rusqlite::Error> {
        let dir = data_local_dir().expect("Failed to get data local directory");
        let dir = dir.join("CuTE");
        let dbpath = dir.join("CuTE.db");

        let conn = Connection::open_with_flags(
            &dbpath,
            OpenFlags::SQLITE_OPEN_READ_WRITE
                | OpenFlags::SQLITE_OPEN_CREATE
                | OpenFlags::SQLITE_OPEN_URI
                | OpenFlags::SQLITE_OPEN_NO_MUTEX,
        )?;

        // Begin a transaction
        conn.execute("BEGIN;", params![])?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS commands (id INTEGER PRIMARY KEY, command TEXT, curl_json TEXT);",
            params![],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS keys (id INTEGER PRIMARY KEY, key TEXT);",
            params![],
        )?;

        conn.execute("COMMIT;", params![])?;

        Ok(DB { conn })
    }

    pub fn add_command(&self, command: &str, json_str: String) -> Result<(), rusqlite::Error> {
        if self.command_exists(command).unwrap() {
            return Ok(());
        }
        self.conn.execute(
            "INSERT INTO commands (command, curl_json) VALUES (?1, ?2)",
            params![command, &json_str],
        )?;
        Ok(())
    }

    pub fn delete_command(&self, command: &str) -> Result<(), rusqlite::Error> {
        self.conn
            .execute("DELETE FROM commands WHERE command = ?1", params![command])?;
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
        if count.is_positive() {
            Ok(true)
        } else {
            Ok(false)
        }
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

    pub fn delete_key(&self, key: &str) -> Result<()> {
        self.conn
            .execute("DELETE FROM keys WHERE key = ?1", params![key])?;
        Ok(())
    }

    pub fn add_key(&self, key: &str) -> Result<()> {
        if !self.key_exists(key).unwrap() {
            return Ok(());
        }
        self.conn
            .execute("INSERT INTO keys (key) VALUES (?1)", params![key])?;
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

impl Display for SavedUrl {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.url)
    }
}

impl Display for SavedCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.command)
    }
}

impl Display for SavedKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ID: {} | Key: {}", self.id, self.key)
    }
}

// TODO: we need to be getting the api key from the command and offering
// to store it separately + link the two. (also encrypt the key?)
// do we use OS keyring or maybe an ENV VAR?
impl SavedCommand {
    // We nned to allow the user to write out the response to a file,
    // so at some point we may need to read it back in
    pub fn to_json(&self) -> Result<String> {
        Ok(serde_json::to_string(&self).expect("Failed to serialize"))
    }

    pub fn from_json(json: &str) -> Result<Self> {
        Ok(serde_json::from_str(json).expect("Failed to deserialize"))
    }

    pub fn get_curl_json(&self) -> String {
        self.curl_json.clone()
    }

    pub fn get_command(&self) -> String {
        self.command.clone()
    }
}

impl SavedKey {
    pub fn new(key: &str) -> Self {
        SavedKey {
            id: 0,
            key: key.to_string(),
        }
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
    //TODO: implement encryption

    // pub fn encrypt(&self, key: &str) -> Result<String> {
    //     let mut encrypted = encrypt(key, self.key.as_str())?;
    //     encrypted.push_str("\n");
    //     Ok(encrypted)
    // }
}
