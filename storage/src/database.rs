use chrono::Utc;
use rusqlite::{Connection, Result};

pub struct Vault {
    pub id: i64,
    pub name: String,
    pub desc: Option<String>,
    pub created_at: i64,
    pub updated_at: i64,
}

pub struct Entry {
    pub id: i64,
    pub access_key: String,
    pub access_token: String,
    pub token_type: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

pub struct Db {
    pub conn: Connection,
}

impl Db {
    pub fn new(path: &str) -> Result<Self> {
        let conn = Connection::open(path)?;
        conn.execute("PRAGMA foreign_keys = ON", [])?;
        Ok(Db { conn })
    }
    pub fn initialize(&self) -> Result<()> {
        self.conn.execute(
            "
            CREATE TABLE IF NOT EXISTS vaults (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL,
                desc TEXT
                created_at INTEGER NOT NULL,
                updated_at INTEGER NOT NULL, 
            )
        ",
            [],
        )?;
        self.conn.execute(
            "
            CREATE TABLE IF NOT EXISTS entries (
                id INTEGER PRIMARY KEY,
                access_key TEXT NOT NULL,
                access_token TEXT NOT NULL,
                token_type TEXT NOT NULL,
                vault_id INTEGER NOT NULL,
                created_at INTEGER NOT NULL, 
                updated_at INTEGER NOT NULL,
                FOREIGN KEY (vault_id) REFERENCES vaults(id)
            )
            ",
            [],
        )?;
        Ok(())
    }
    pub fn add_new_vault(&self, name: &str, desc: Option<&str>) -> Result<()> {
        let time = Utc::now().timestamp();
        self.conn.execute(
            "
            INSERT INTO vaults (name,desc,created_at,updated_at)
            VALUES(?1,?2,?3,?4)
            ",
            (name, desc, &time, &time),
        )?;
        Ok(())
    }

    pub fn get_vaults(&self) -> Result<Vec<Vault>> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, name, desc,created_at,updated_at FROM vaults")?;

        let vaults = stmt.query_map([], |row| {
            Ok(Vault {
                id: row.get(0)?,
                name: row.get(1)?,
                desc: row.get(2)?,
                created_at: row.get(3)?,
                updated_at: row.get(4)?,
            })
        })?;
        let mut result = Vec::new();

        for vault in vaults {
            result.push(vault?);
        }
        Ok(result)
    }
}
