use crate::file::record::{RecordType, TlvRecord};
use std::io;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Entry {
    records: Vec<TlvRecord>,
}

impl Entry {
    pub fn new() -> Self {
        let uuid = Uuid::new_v4();
        let mut entry = Self {
            records: Vec::new(),
        };
        entry.add_record(TlvRecord::new(
            super::record::RecordType::Uuid,
            uuid.as_bytes().to_vec(),
        ));

        entry
    }
    pub fn records(&self) -> &[TlvRecord] {
        &self.records
    }
    pub fn add_record(&mut self, record: TlvRecord) {
        self.records.push(record);
    }
    pub fn serialize(&self) -> Vec<u8> {
        let mut buffer = Vec::new();

        for record in &self.records {
            buffer.extend_from_slice(&record.serialize());
        }

        buffer
    }
    pub fn deserialize(bytes: &[u8]) -> io::Result<Self> {
        let mut records = Vec::new();
        let mut offset = 0;
        while offset < bytes.len() {
            let (record, used) = TlvRecord::deserialize(&bytes[offset..])?;
            records.push(record);
            offset += used;
        }

        Ok(Self { records })
    }
    pub fn from_fields(
        title: &str,
        username: &str,
        password: &str,
        url: &str,
        notes: &str,
    ) -> Self {
        let mut entry = Self::new();

        entry.set_title(title);
        entry.set_username(username);
        entry.set_password(password);
        entry.set_url(url);
        entry.set_notes(notes);

        entry
    }
    fn set_field(&mut self, record_type: RecordType, value: &str) {
        if let Some(record) = self
            .records
            .iter_mut()
            .find(|record| record.record_type == record_type)
        {
            record.value = value.as_bytes().to_vec();
            return;
        }
        self.add_record(TlvRecord::new(record_type, value.as_bytes().to_vec()));
    }
    pub fn set_title(&mut self, title: &str) {
        self.set_field(RecordType::Title, title);
    }
    pub fn set_username(&mut self, username: &str) {
        self.set_field(RecordType::Username, username);
    }
    pub fn set_password(&mut self, password: &str) {
        self.set_field(RecordType::Password, password);
    }
    pub fn set_url(&mut self, url: &str) {
        self.set_field(RecordType::Url, url);
    }
    pub fn set_notes(&mut self, notes: &str) {
        self.set_field(RecordType::Notes, notes);
    }
    pub fn uuid(&self) -> io::Result<Uuid> {
        let record = self
            .records
            .iter()
            .find(|record| record.record_type == RecordType::Uuid)
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "entry has no UUID."))?;

        Uuid::from_slice(&record.value)
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "invalid UUID."))
    }
    pub fn title(&self) -> Option<&[u8]> {
        self.records
            .iter()
            .find(|record| record.record_type == RecordType::Title)
            .map(|record| record.value.as_slice())
    }
    pub fn username(&self) -> Option<&[u8]> {
        self.records
            .iter()
            .find(|record| record.record_type == RecordType::Username)
            .map(|record| record.value.as_slice())
    }

    pub fn password(&self) -> Option<&[u8]> {
        self.records
            .iter()
            .find(|record| record.record_type == RecordType::Password)
            .map(|record| record.value.as_slice())
    }

    pub fn url(&self) -> Option<&[u8]> {
        self.records
            .iter()
            .find(|record| record.record_type == RecordType::Url)
            .map(|record| record.value.as_slice())
    }

    pub fn notes(&self) -> Option<&[u8]> {
        self.records
            .iter()
            .find(|record| record.record_type == RecordType::Notes)
            .map(|record| record.value.as_slice())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::file::record::RecordType;

    #[test]
    fn entry_roundtrip() {
        let mut entry = Entry::new();

        entry.add_record(TlvRecord::new(RecordType::Title, b"GitHub".to_vec()));

        entry.add_record(TlvRecord::new(RecordType::Username, b"user123".to_vec()));

        let uuid = entry.uuid().unwrap();

        let bytes = entry.serialize();

        let decoded = Entry::deserialize(&bytes).unwrap();

        assert_eq!(decoded.records.len(), 3);
        assert_eq!(decoded.records[0].record_type, RecordType::Uuid);
        assert_eq!(decoded.records[1].record_type, RecordType::Title);
        assert_eq!(decoded.records[2].record_type, RecordType::Username);

        assert_eq!(decoded.uuid().unwrap(), uuid);
    }
    #[test]
    fn entry_has_uuid() {
        let entry = Entry::new();

        let uuid_record = entry
            .records()
            .iter()
            .find(|record| record.record_type == RecordType::Uuid);

        assert!(uuid_record.is_some());

        let uuid_record = uuid_record.unwrap();

        assert_eq!(uuid_record.value.len(), 16);
    }
    #[test]
    fn entry_uuid_is_unique() {
        let entry1 = Entry::new();
        let entry2 = Entry::new();

        let uuid1 = entry1.uuid().unwrap();
        let uuid2 = entry2.uuid().unwrap();

        assert_ne!(uuid1, uuid2);
    }
    #[test]
    fn entry_field_getters() {
        let entry = Entry::from_fields(
            "GitHub",
            "user123",
            "secret",
            "https://github.com",
            "My GitHub account",
        );

        assert_eq!(entry.title(), Some(b"GitHub".as_slice()));
        assert_eq!(entry.username(), Some(b"user123".as_slice()));
        assert_eq!(entry.password(), Some(b"secret".as_slice()));
        assert_eq!(entry.url(), Some(b"https://github.com".as_slice()));
        assert_eq!(entry.notes(), Some(b"My GitHub account".as_slice()));
    }
    #[test]
    fn entry_set_fields() {
        let mut entry = Entry::from_fields(
            "GitHub",
            "old-user",
            "old-password",
            "https://old.example",
            "old notes",
        );

        let uuid = entry.uuid().unwrap();

        entry.set_title("GitHub Personal");
        entry.set_username("new-user");
        entry.set_password("new-password");
        entry.set_url("https://new.example");
        entry.set_notes("new notes");

        assert_eq!(entry.uuid().unwrap(), uuid);

        assert_eq!(entry.title(), Some(b"GitHub Personal".as_slice()));
        assert_eq!(entry.username(), Some(b"new-user".as_slice()));
        assert_eq!(entry.password(), Some(b"new-password".as_slice()));
        assert_eq!(entry.url(), Some(b"https://new.example".as_slice()));
        assert_eq!(entry.notes(), Some(b"new notes".as_slice()));
    }
}
