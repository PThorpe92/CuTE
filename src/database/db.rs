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

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct SavedCommand {
    pub id: i32,
    command: String,
    pub description: Option<String>,
    pub label: Option<String>,
    curl_json: String,
    pub collection_id: Option<i32>,
    pub collection_name: Option<String>,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct SavedCollection {
    id: i32,
    pub name: String,
    pub description: Option<String>,
}

impl Display for SavedCollection {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct SavedKey {
    id: i32,
    label: Option<String>,
    key: String,
}

#[derive(Debug)]
pub struct DB {
    conn: Connection,
}

impl DB {
    pub fn new_test() -> Result<Self, rusqlite::Error> {
        let conn = Connection::open_in_memory()?;
        conn.execute(
            "CREATE TABLE commands (id INTEGER PRIMARY KEY, command TEXT, label TEXT, description TEXT, curl_json TEXT, collection_id INT);",
            params![],
        )?;
        conn.execute(
            "CREATE TABLE keys (id INTEGER PRIMARY KEY, key TEXT, label TEXT);",
            params![],
        )?;
        conn.execute(
            "CREATE TABLE collections (id INTEGER PRIMARY KEY, name TEXT, description TEXT);",
            params![],
        )?;
        Ok(DB { conn })
    }

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

        let conn = match conn_result {
            Ok(connection) => connection,
            Err(e) => {
                println!("CuTE Database Error: {:?}", e);
                return Err(e);
            }
        };

        // Begin a transaction
        conn.execute("BEGIN;", params![])?;
        // collection_id needs to be nullable
        conn.execute(
            "CREATE TABLE IF NOT EXISTS commands (id INTEGER PRIMARY KEY, label TEXT, description TEXT, command TEXT, curl_json TEXT, collection_id INT);",
            params![],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS keys (id INTEGER PRIMARY KEY, key TEXT, label TEXT);",
            params![],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS collections (id INTEGER PRIMARY KEY, name TEXT, description TEXT);",
            params![],
        )?;

        conn.execute("COMMIT;", params![])?;

        Ok(DB { conn })
    }

    pub fn rename_collection(&self, id: i32, name: &str) -> Result<(), rusqlite::Error> {
        let mut stmt = self
            .conn
            .prepare("UPDATE collections SET name = ? WHERE id = ?")?;
        stmt.execute(params![name, id])?;
        Ok(())
    }

    pub fn set_collection_description(
        &self,
        id: i32,
        description: &str,
    ) -> Result<(), rusqlite::Error> {
        let mut stmt = self
            .conn
            .prepare("UPDATE collections SET description = ? WHERE id = ?")?;
        stmt.execute(params![description, id])?;
        Ok(())
    }

    pub fn get_number_of_commands_in_collection(&self, id: i32) -> Result<i32> {
        let mut stmt = self
            .conn
            .prepare("SELECT COUNT(*) FROM commands WHERE collection_id = ?")?;
        let count: i32 = stmt.query_row(params![id], |row| row.get(0))?;
        Ok(count)
    }

#[rustfmt::skip]
    pub fn add_collection(&self, name: &str, desc: &str, commands: &[SavedCommand]) -> Result<(), Box<dyn std::error::Error>> {
        let mut stmt = self
            .conn
            .prepare("INSERT INTO collections (name, description) VALUES (?1, ?2)")?;
        let id = stmt.insert(params![name, desc])?;
        for command in commands {
            self.add_command_from_collection(&command.command, command.label.as_deref(), command.description.as_deref(), &command.curl_json, id as i32)?;
        }
        Ok(())
    }

    pub fn get_command_by_id(&self, id: i32) -> Result<SavedCommand> {
        let mut stmt = self.conn.prepare(
                "SELECT cmd.id, cmd.command, cmd.label, cmd.description, cmd.curl_json, cmd.collection_id, col.name as collection_name FROM commands cmd LEFT JOIN collections col ON cmd.collection_id = col.id WHERE cmd.id = ?"
        )?;
        stmt.query_row(params![id], |row| {
            Ok(SavedCommand {
                id: row.get(0)?,
                command: row.get(1)?,
                label: row.get(2)?,
                description: row.get(3)?,
                curl_json: row.get(4)?,
                collection_id: row.get(5)?,
                collection_name: row.get(6)?,
            })
        })
    }

    pub fn get_collection_by_id(&self, id: i32) -> Result<SavedCollection, String> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, name, description FROM collections WHERE id = ?")
            .map_err(|_| "No Collection".to_string())?;
        let collection = stmt
            .query_row(params![id], |row| {
                Ok(SavedCollection {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    description: row.get(2)?,
                })
            })
            .map_err(|_| ("No Collection".to_string()))?;
        Ok(collection)
    }

    pub fn create_collection(&self, name: &str) -> Result<(), rusqlite::Error> {
        let mut stmt = self
            .conn
            .prepare("INSERT INTO collections (name) VALUES (?1)")?;
        stmt.execute(params![name])?;
        Ok(())
    }

    pub fn get_collections(&self) -> Result<Vec<SavedCollection>> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, name, description FROM collections")?;
        let rows = stmt.query_map(params![], |row| {
            Ok(SavedCollection {
                id: row.get(0)?,
                name: row.get(1)?,
                description: row.get(2)?,
            })
        })?;
        Ok(rows
            .into_iter()
            .filter_map(|row| row.ok())
            .collect::<Vec<_>>())
    }

    pub fn get_default_path() -> PathBuf {
        let dir = data_local_dir().expect("Failed to get data local directory,\nPlease specify a path at $CONFIG/CuTE/config.toml\nOr with the --db_path={path/to/CuTE.db}");
        dir.join("CuTE")
    }

    pub fn add_command(
        &self,
        command: &str,
        json_str: String,
        col_id: Option<i32>,
    ) -> Result<(), rusqlite::Error> {
        let mut stmt = self.conn.prepare(
            "INSERT INTO commands (command, curl_json, collection_id) VALUES (?1, ?2, ?3)",
        )?;
        let _ = stmt.execute(params![command, &json_str, col_id])?;
        Ok(())
    }

    pub fn set_command_description(
        &self,
        id: i32,
        description: &str,
    ) -> Result<Option<i32>, rusqlite::Error> {
        let mut stmt = self
            .conn
            .prepare("UPDATE commands SET description = ?1 WHERE id = ?2")?;
        stmt.execute(params![description, id])?;
        let mut stmt = self
            .conn
            .prepare("SELECT collection_id FROM commands WHERE id = ?")?;
        let collection_id: Option<i32> = stmt.query_row(params![id], |row| row.get(0))?;
        Ok(collection_id)
    }

    pub fn set_command_label(&self, id: i32, label: &str) -> Result<Option<i32>, rusqlite::Error> {
        let mut stmt = self
            .conn
            .prepare("UPDATE commands SET label = ?1 WHERE id = ?2")?;
        stmt.execute(params![label, id])?;
        let mut stmt = self
            .conn
            .prepare("SELECT collection_id FROM commands WHERE id = ?")?;
        let collection_id: Option<i32> = stmt.query_row(params![id], |row| row.get(0))?;
        Ok(collection_id)
    }

    pub fn add_command_from_collection(
        &self,
        command: &str,
        label: Option<&str>,
        desc: Option<&str>,
        json_str: &str,
        collection_id: i32,
    ) -> Result<(), rusqlite::Error> {
        let mut stmt = self.conn.prepare(
            "INSERT INTO commands (command, label, description, curl_json, collection_id) VALUES (?1, ?2, ?3, ?4, ?5)",
        )?;
        let _ = stmt.execute(params![command, label, desc, json_str, collection_id])?;
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

    pub fn get_commands(&self, id: Option<i32>) -> Result<Vec<SavedCommand>> {
        if let Some(id) = id {
            let mut stmt = self
                .conn
                .prepare("SELECT cmd.id, cmd.command, cmd.label, cmd.description, cmd.curl_json, cmd.collection_id, col.name as collection_name FROM commands cmd LEFT JOIN collections col ON cmd.collection_id = col.id WHERE cmd.collection_id = ?")?;
            let rows = stmt.query_map(params![id], |row| {
                Ok(SavedCommand {
                    id: row.get(0)?,
                    command: row.get(1)?,
                    label: row.get(2)?,
                    description: row.get(3)?,
                    curl_json: row.get(4)?,
                    collection_id: row.get(5)?,
                    collection_name: row.get(6)?,
                })
            })?;
            return Ok(rows.into_iter().filter_map(|row| row.ok()).collect());
        }
        let mut stmt = self
            .conn
            .prepare("SELECT cmd.id, cmd.command, cmd.label, cmd.description, cmd.curl_json, cmd.collection_id, col.name FROM commands cmd LEFT JOIN collections col ON cmd.collection_id = col.id")?;
        let rows = stmt.query_map(params![], |row| {
            Ok(SavedCommand {
                id: row.get(0)?,
                command: row.get(1)?,
                label: row.get(2)?,
                description: row.get(3)?,
                curl_json: row.get(4)?,
                collection_id: row.get(5)?,
                collection_name: row.get(6)?,
            })
        })?;
        let mut commands = Vec::new();
        rows.for_each(|row| {
            commands.push(row.unwrap_or_default());
        });
        Ok(commands)
    }

    pub fn delete_collection(&self, id: i32) -> Result<(), rusqlite::Error> {
        let mut stmt = self.conn.prepare("DELETE FROM collections WHERE id = ?")?;
        stmt.execute([id])?;
        let mut stmt = self
            .conn
            .prepare("DELETE FROM commands WHERE collection_id = ?")?;
        stmt.execute([id])?;
        Ok(())
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

    pub fn set_key_label(&self, key: i32, label: &str) -> Result<()> {
        let mut stmt = self
            .conn
            .prepare("UPDATE keys SET label = ?1 WHERE id = ?2")?;
        stmt.execute(params![label, key])?;
        Ok(())
    }

    pub fn get_keys(&self) -> Result<Vec<SavedKey>> {
        let mut stmt = self.conn.prepare("SELECT id, key, label FROM keys")?;
        let rows = stmt.query_map(params![], |row| {
            Ok(SavedKey {
                id: row.get(0)?,
                key: row.get(1)?,
                label: row.get(2)?,
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

    pub fn new(
        command: &str,
        label: Option<String>,
        description: Option<String>,
        curl_json: &str,
        collection_id: Option<i32>,
    ) -> Self {
        SavedCommand {
            command: command.to_string(),
            label,
            curl_json: curl_json.to_string(),
            collection_id,
            description,
            ..Default::default()
        }
    }

    pub fn get_id(&self) -> i32 {
        self.id
    }

    pub fn from_json(json: &str) -> Result<Self> {
        Ok(serde_json::from_str(json).expect("Failed to deserialize"))
    }

    pub fn get_collection_id(&self) -> Option<i32> {
        self.collection_id
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
        if self.label.is_some() {
            write!(f, "Label: {:?} |  : {}", self.get_label(), self.key)
        } else {
            write!(f, " : {}", self.key)
        }
    }
}

impl SavedKey {
    pub fn new(key: &str) -> Self {
        SavedKey {
            id: 0,
            key: key.to_string(),
            label: None,
        }
    }

    pub fn get_label(&self) -> &str {
        match &self.label {
            Some(label) => label,
            None => "",
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

impl SavedCollection {
    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn get_id(&self) -> i32 {
        self.id
    }
}
