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
                desc TEXT,
                created_at INTEGER NOT NULL,
                updated_at INTEGER NOT NULL
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
            INSERT INTO vaults (name, desc, created_at, updated_at)
            VALUES(?1,?2,?3,?4)
            ",
            (name, desc, &time, &time),
        )?;
        Ok(())
    }

    pub fn get_vaults(&self) -> Result<Vec<Vault>> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, name, desc, created_at, updated_at FROM vaults")?;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_database_initialization() {
        let db = Db::new(":memory:").unwrap();
        let result = db.initialize();
        assert!(result.is_ok());
    }

    #[test]
    fn test_empty_vaults() {
        let db = Db::new(":memory:").unwrap();
        db.initialize().unwrap();
        let vaults = db.get_vaults().unwrap();
        assert_eq!(vaults.len(), 0);
    }

    #[test]
    fn test_multiple_vaults() {
        let db = Db::new(":memory:").unwrap();
        db.initialize().unwrap();
        db.add_new_vault("Personal", None).unwrap();
        db.add_new_vault("Work", None).unwrap();
        let vaults = db.get_vaults().unwrap();

        assert_eq!(vaults.len(), 2);
        assert_eq!(vaults[0].name, "Personal");
        assert_eq!(vaults[1].name, "Work");
    }

    #[test]
    fn test_vault_with_description() {
        let db = Db::new(":memory:").unwrap();
        db.initialize().unwrap();
        db.add_new_vault("Personal", Some("My personal accounts"))
            .unwrap();
        let vaults = db.get_vaults().unwrap();
        assert_eq!(vaults[0].desc, Some("My personal accounts".to_string()));
    }

    #[test]
    fn test_foreign_key_constraint() {
        let db = Db::new(":memory:").unwrap();
        db.initialize().unwrap();
        let result = db.conn.execute(
            "
            INSERT INTO entries (
                access_key,
                access_token,
                token_type,
                vault_id,
                created_at,
                updated_at
            )
            VALUES (?1, ?2, ?3)
            ",
            ("username", "password", "text", 999, 23, 23),
        );
        assert!(result.is_err());
    }
}
