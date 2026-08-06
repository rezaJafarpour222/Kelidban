use rusqlite::{Connection, Result};

pub struct Vault {
    pub id: Option<i64>,
    pub name: String,
    pub desc: Option<String>,
    pub created_at: i64,
    pub updated_at: i64,
}
#[derive(Debug)]
pub struct Entry {
    pub id: Option<i64>,
    pub access_key: String,
    pub access_token: String,
    pub token_type: String,
    pub desc: Option<String>,
    pub created_at: i64,
    pub updated_at: i64,
    pub vault_id: i64,
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
                desc TEXT,
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
    pub fn add_vault(&self, vault: &Vault) -> Result<()> {
        self.conn.execute(
            "
            INSERT INTO vaults (name, desc, created_at, updated_at)
            VALUES(?1,?2,?3,?4)
            ",
            (
                &vault.name,
                &vault.desc,
                &vault.created_at,
                &vault.updated_at,
            ),
        )?;
        Ok(())
    }

    pub fn add_entry(&self, entry: &Entry) -> Result<()> {
        self.conn.execute(
            "
            INSERT INTO entries (access_key, access_token, token_type, desc, created_at, updated_at, vault_id)
            VALUES(?1,?2,?3,?4,?5,?6,?7)
            ",
            (
                &entry.access_key,
                &entry.access_token,
                &entry.token_type,
                &entry.desc,
                &entry.created_at,
                &entry.updated_at,
                &entry.vault_id
            ),
        )?;
        Ok(())
    }

    pub fn get_vaults(&self) -> Result<Vec<Vault>> {
        let mut stmt = self.conn.prepare("SELECT * FROM vaults")?;
        let vaults = stmt.query_map([], |row| {
            Ok(Vault {
                id: row.get("id")?,
                name: row.get("name")?,
                desc: row.get("desc")?,
                created_at: row.get("created_at")?,
                updated_at: row.get("updated_at")?,
            })
        })?;
        vaults.collect()
    }
    pub fn get_entries_from_vault(&self, vault_id: i64) -> Result<Vec<Entry>> {
        let mut stmt = self
            .conn
            .prepare(" SELECT * FROM entries WHERE vault_id =?1")?;

        let entries = stmt.query_map([vault_id], |row| {
            Ok(Entry {
                id: row.get("id")?,
                access_key: row.get("access_key")?,
                access_token: row.get("access_token")?,
                token_type: row.get("token_type")?,
                desc: row.get("desc")?,
                created_at: row.get("created_at")?,
                updated_at: row.get("updated_at")?,
                vault_id: row.get("vault_id")?,
            })
        })?;
        entries.collect()
    }
    pub fn get_vault(&self, vault_id: i64) -> Result<Vault> {
        let mut stmt = self.conn.prepare("SELECT * FROM vaults WHERE id = ?1")?;
        let vault = stmt.query_row([vault_id], |row| {
            Ok(Vault {
                id: row.get("id")?,
                name: row.get("name")?,
                desc: row.get("desc")?,
                created_at: row.get("created_at")?,
                updated_at: row.get("updated_at")?,
            })
        })?;
        Ok(vault)
    }
    pub fn get_entry(&self, entry_id: i64) -> Result<Entry> {
        let mut stmt = self.conn.prepare("SELECT * FROM entries WHERE id = ?1")?;
        let entry = stmt.query_row([entry_id], |row| {
            Ok(Entry {
                id: row.get("id")?,
                access_key: row.get("access_key")?,
                access_token: row.get("access_token")?,
                token_type: row.get("token_type")?,
                desc: row.get("desc")?,
                created_at: row.get("created_at")?,
                updated_at: row.get("updated_at")?,
                vault_id: row.get("vault_id")?,
            })
        })?;
        Ok(entry)
    }
}

#[cfg(test)]
mod tests {

    use chrono::Utc;

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
        let time = Utc::now().timestamp();
        let v1 = Vault {
            id: None,
            name: String::from("Personal"),
            desc: Some(String::from("My Personal Account")),
            created_at: time,
            updated_at: time,
        };

        let v2 = Vault {
            id: None,
            name: String::from("Work"),
            desc: Some(String::from("My Work Account")),
            created_at: time,
            updated_at: time,
        };
        db.add_vault(&v1).unwrap();
        db.add_vault(&v2).unwrap();
        let vaults = db.get_vaults().unwrap();

        assert_eq!(vaults.len(), 2);
        assert_eq!(vaults[0].name, "Personal");
        assert_eq!(vaults[0].desc, Some(String::from("My Personal Account")));
        assert_eq!(vaults[0].created_at, time);
        assert_eq!(vaults[0].updated_at, time);
        assert_eq!(vaults[1].name, "Work");
        assert_eq!(vaults[1].desc, Some(String::from("My Work Account")));
        assert_eq!(vaults[1].created_at, time);
        assert_eq!(vaults[1].updated_at, time);
    }

    #[test]
    fn test_vault_with_description() {
        let db = Db::new(":memory:").unwrap();
        db.initialize().unwrap();
        let time = Utc::now().timestamp();
        let v = Vault {
            id: None,
            name: String::from("Personal"),
            desc: Some(String::from("My Personal Account")),
            created_at: time,
            updated_at: time,
        };
        db.add_vault(&v).unwrap();
        let vaults = db.get_vaults().unwrap();
        assert_eq!(vaults[0].desc, Some(String::from("My Personal Account")));
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
                desc,
                vault_id,
                created_at,
                updated_at
            )
            VALUES (?1, ?2, ?3,?4,?5,?6)
            ",
            ("username", "password", "text", 999, 23, 23),
        );
        assert!(result.is_err());
    }
    #[test]
    fn get_single_vault() {
        let db = Db::new(":memory:").unwrap();
        db.initialize().unwrap();
        let time = Utc::now().timestamp();
        let v = Vault {
            id: None,
            name: String::from("Personal"),
            desc: Some(String::from("My Personal Account")),
            created_at: time,
            updated_at: time,
        };
        db.add_vault(&v).unwrap();
        let vault = db.get_vault(1).unwrap();
        assert_eq!(v.name, vault.name);
        assert_eq!(v.desc, vault.desc);
        assert_eq!(v.created_at, vault.created_at);
        assert_eq!(v.updated_at, vault.updated_at);
    }

    #[test]
    fn get_single_entry() {
        let db = Db::new(":memory:").unwrap();
        db.initialize().unwrap();

        let time = Utc::now().timestamp();
        let v1 = Vault {
            id: None,
            name: String::from("Work"),
            desc: Some(String::from("My Work Account")),
            created_at: time,
            updated_at: time,
        };
        db.add_vault(&v1).unwrap();

        let e = Entry {
            id: None,
            access_key: String::from("ansible"),
            access_token: String::from("TOPSCERETpassWord23123214"),
            token_type: String::from("SSH"),
            desc: None,
            created_at: time,
            updated_at: time,
            vault_id: 1,
        };
        db.add_entry(&e).unwrap();
        let entry = db.get_entry(1).unwrap();
        assert_eq!(1, entry.vault_id);
        assert_eq!(e.access_key, entry.access_key);
        assert_eq!(e.access_token, entry.access_token);
        assert_eq!(e.token_type, entry.token_type);
        assert_eq!(e.desc, entry.desc);
        assert_eq!(e.created_at, entry.created_at);
        assert_eq!(e.updated_at, entry.updated_at);
    }
}
