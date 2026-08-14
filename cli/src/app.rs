use std::{io, path::Path};

use encryption::file::{
    entry::Entry,
    storage::{load_vault, save_vault},
    vault::Vault,
};
use uuid::Uuid;

pub struct App {
    vault: Vault,
}
impl App {
    pub fn new(vault: Vault) -> Self {
        Self { vault }
    }
    pub fn add_entry(
        &mut self,
        title: &str,
        username: &str,
        password: &str,
        url: &str,
        notes: &str,
    ) {
        self.vault
            .add_entry_from_fields(title, username, password, url, notes);
    }
    pub fn find_entry(&self, uuid: Uuid) -> io::Result<Option<&Entry>> {
        self.vault.find_entry(uuid)
    }
    pub fn delete_entry(&mut self, uuid: Uuid) -> io::Result<()> {
        self.vault.delete_entry(uuid)
    }
    pub fn entries(&self) -> &[Entry] {
        self.vault.entries()
    }
    pub fn update_entry(
        &mut self,
        uuid: Uuid,
        title: &str,
        username: &str,
        password: &str,
        url: &str,
        notes: &str,
    ) -> io::Result<()> {
        let entry = self
            .vault
            .find_entry_mut(uuid)?
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "entry not found."))?;

        entry.set_title(title);
        entry.set_username(username);
        entry.set_password(password);
        entry.set_url(url);
        entry.set_notes(notes);

        Ok(())
    }
    // app.rs

    pub fn save<P: AsRef<Path>>(&self, path: P, password: &[u8]) -> io::Result<()> {
        save_vault(path, password, &self.vault)
    }

    pub fn load<P: AsRef<Path>>(path: P, password: &[u8]) -> io::Result<Self> {
        let vault = load_vault(path, password)?;
        Ok(Self::new(vault))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_entry() {
        let vault = Vault::new();
        let mut app = App::new(vault);

        app.add_entry(
            "GitHub",
            "user123",
            "secret",
            "https://github.com",
            "My GitHub account",
        );

        assert_eq!(app.entries().len(), 1);

        let entry = &app.entries()[0];

        assert_eq!(entry.title(), Some(b"GitHub".as_slice()));
        assert_eq!(entry.username(), Some(b"user123".as_slice()));
        assert_eq!(entry.password(), Some(b"secret".as_slice()));
        assert_eq!(entry.url(), Some(b"https://github.com".as_slice()));
        assert_eq!(entry.notes(), Some(b"My GitHub account".as_slice()));
    }
    #[test]
    fn get_all_entries() {
        let mut app = App::new(Vault::new());

        app.add_entry(
            "GitHub",
            "user123",
            "secret",
            "https://github.com",
            "My GitHub account",
        );

        app.add_entry(
            "Bank",
            "bankuser",
            "bankpass",
            "https://bank.example",
            "Bank account",
        );

        assert_eq!(app.entries().len(), 2);
        assert_eq!(app.entries()[0].title(), Some(b"GitHub".as_slice()));
        assert_eq!(app.entries()[1].title(), Some(b"Bank".as_slice()));
    }
    #[test]
    fn update_entry() {
        let mut app = App::new(Vault::new());

        app.add_entry(
            "GitHub",
            "old-user",
            "old-password",
            "https://github.com",
            "old notes",
        );

        let uuid = app.entries()[0].uuid().unwrap();

        app.update_entry(
            uuid,
            "GitHub Personal",
            "new-user",
            "new-password",
            "https://github.com/new",
            "new notes",
        )
        .unwrap();

        let entry = app.find_entry(uuid).unwrap().unwrap();

        assert_eq!(entry.uuid().unwrap(), uuid);
        assert_eq!(entry.title(), Some(b"GitHub Personal".as_slice()));
        assert_eq!(entry.username(), Some(b"new-user".as_slice()));
        assert_eq!(entry.password(), Some(b"new-password".as_slice()));
        assert_eq!(entry.url(), Some(b"https://github.com/new".as_slice()));
        assert_eq!(entry.notes(), Some(b"new notes".as_slice()));
    }
    #[test]
    fn delete_entry() {
        let mut app = App::new(Vault::new());

        app.add_entry(
            "GitHub",
            "user123",
            "secret",
            "https://github.com",
            "My GitHub account",
        );

        app.add_entry(
            "Bank",
            "bankuser",
            "bankpass",
            "https://bank.example",
            "Bank account",
        );

        let first_uuid = app.entries()[0].uuid().unwrap();
        let second_uuid = app.entries()[1].uuid().unwrap();

        app.delete_entry(first_uuid).unwrap();

        assert_eq!(app.entries().len(), 1);

        let remaining = app.find_entry(second_uuid).unwrap();

        assert!(remaining.is_some());
        assert_eq!(remaining.unwrap().title(), Some(b"Bank".as_slice()));
    }
}
