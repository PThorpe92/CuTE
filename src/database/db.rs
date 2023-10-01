use chrono::Utc;
use rusqlite::{params, Connection, Result};
use serde::{Deserialize, Serialize};
use serde_json;

#[cfg(not(target_os = "windows"))]
pub const USER_DB_PATH: &str = "/home/$USER/.local/share/CuTE/CuTE.sqlite";

#[cfg(target_os = "windows")]
pub const USER_DB_PATH: &str = "C:\\Users\\$USER\\AppData\\Local\\CuTE\\CuTE.sqlite";

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SavedCommand {
    pub id: i64,
    pub command: String,
    pub timestamp: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SavedKey {
    pub id: i64,
    pub key: String,
    pub timestamp: String,
}

#[derive(Debug)]
pub struct DB {
    pub conn: Connection,
}

impl DB {
    pub fn new(path: &str) -> Result<Self, rusqlite::Error> {
        let conn = Connection::open(path)?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS commands (
            id INTEGER PRIMARY KEY,
            command TEXT NOT NULL,
            created_at TEXT NOT NULL
            )",
            params![],
        )?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS keys (
            id INTEGER PRIMARY KEY,
            key TEXT NOT NULL,
            created_at TEXT NOT NULL
                )",
            params![],
        )?;
        Ok(DB { conn })
    }

    pub fn add_command(&self, command: &str) -> Result<(), rusqlite::Error> {
        let timestamp = Utc::now().to_rfc3339();
        self.conn.execute(
            "INSERT INTO commands (command, timestamp) VALUES (?1, ?2)",
            params![command, timestamp],
        )?;
        Ok(())
    }

    pub fn get_commands(&self) -> Result<Vec<SavedCommand>> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, command, timestamp FROM commands")?;
        let rows = stmt.query_map(params![], |row| {
            Ok(SavedCommand {
                id: row.get(0)?,
                command: row.get(1)?,
                timestamp: row.get(2)?,
            })
        })?;
        let mut commands = Vec::new();
        rows.for_each(|row| {
            commands.push(row.unwrap());
        });
        Ok(commands)
    }

    pub fn add_key(&self, key: &str) -> Result<()> {
        self.conn
            .execute("INSERT INTO keys (key) VALUES (?1)", params![key])?;
        Ok(())
    }

    pub fn get_keys(&self) -> Result<Vec<SavedKey>> {
        let mut stmt = self.conn.prepare("SELECT id, key FROM keys")?;
        let timestamp = Utc::now().to_rfc3339();
        let rows = stmt.query_map(params![], |row| {
            Ok(SavedKey {
                id: row.get(0)?,
                key: row.get(1)?,
                timestamp: timestamp.clone(),
            })
        })?;
        let mut keys = Vec::new();
        for key in rows {
            keys.push(key?);
        }
        Ok(keys)
    }
}

impl SavedCommand {
    // We nned to allow the user to write out the response to a file,
    // so at some point we may need to read it back in
    pub fn to_json(&self) -> Result<String> {
        Ok(serde_json::to_string(&self).expect("Failed to serialize"))
    }

    pub fn from_json(json: &str) -> Result<Self> {
        Ok(serde_json::from_str(json).expect("Failed to deserialize"))
    }
}

impl SavedKey {
    pub fn new(key: &str) -> Self {
        SavedKey {
            id: 0,
            key: key.to_string(),
            timestamp: Utc::now().to_rfc3339(),
        }
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
