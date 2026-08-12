use std::io;

use uuid::Uuid;

use crate::file::entry::Entry;

#[derive(Debug, Clone)]
pub struct Vault {
    entries: Vec<Entry>,
}
impl Vault {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }
    pub fn serialize(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        buffer.extend_from_slice(&(self.entries.len() as u32).to_le_bytes());
        for entry in &self.entries {
            let data = entry.serialize();
            buffer.extend_from_slice(&(data.len() as u32).to_le_bytes());
            buffer.extend_from_slice(&data);
        }
        buffer
    }
    pub fn deserialize(bytes: &[u8]) -> io::Result<Self> {
        if bytes.len() < 4 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "vault too small",
            ));
        }
        let mut offset = 0;
        let entry_count =
            u32::from_le_bytes(bytes[offset..offset + 4].try_into().unwrap()) as usize;
        offset += 4;

        let mut entries = Vec::new();
        for _ in 0..entry_count {
            if bytes.len() < offset + 4 {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "missing entry size",
                ));
            }
            let entry_size =
                u32::from_le_bytes(bytes[offset..offset + 4].try_into().unwrap()) as usize;
            offset += 4;

            if bytes.len() < offset + entry_size {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "invalid entry size",
                ));
            }
            let entry = Entry::deserialize(&bytes[offset..offset + entry_size])?;
            entries.push(entry);
            offset += entry_size;
        }
        Ok(Self { entries })
    }
    pub fn add_entry(&mut self, entry: Entry) {
        self.entries.push(entry);
    }
    pub fn entries(&self) -> &[Entry] {
        &self.entries
    }
    pub fn update_entry(&mut self, uuid: Uuid, entry: Entry) -> io::Result<()> {
        let index = self
            .entries
            .iter()
            .position(|entry| entry.uuid().ok() == Some(uuid))
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "entry not found"))?;

        if entry.uuid()? != uuid {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "replacement entry has a different UUID",
            ));
        }

        self.entries[index] = entry;

        Ok(())
    }
    pub fn delete_entry(&mut self, uuid: Uuid) -> io::Result<()> {
        let index = self
            .entries
            .iter()
            .position(|entry| entry.uuid().ok() == Some(uuid))
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "entry not found"))?;

        self.entries.remove(index);

        Ok(())
    }
    pub fn find_entry(&self, uuid: Uuid) -> io::Result<Option<&Entry>> {
        for entry in &self.entries {
            if entry.uuid()? == uuid {
                return Ok(Some(entry));
            }
        }
        Ok(None)
    }
}
#[cfg(test)]
mod tests {
    use crate::file::record::{RecordType, TlvRecord};

    use super::*;
    #[test]
    fn vault_roundtrip() {
        let mut vault = Vault::new();

        let mut entry = Entry::new();
        entry.add_record(TlvRecord::new(RecordType::Title, b"GitHub".to_vec()));

        vault.add_entry(entry);

        let bytes = vault.serialize();

        let decoded = Vault::deserialize(&bytes).unwrap();

        assert_eq!(decoded.entries().len(), 1);
    }
    #[test]
    fn vault_multiple_entries_roundtrip() {
        let mut vault = Vault::new();

        let mut entry1 = Entry::new();
        entry1.add_record(TlvRecord::new(RecordType::Title, b"GitHub".to_vec()));

        let mut entry2 = Entry::new();
        entry2.add_record(TlvRecord::new(RecordType::Title, b"Bank".to_vec()));

        vault.add_entry(entry1);
        vault.add_entry(entry2);

        let bytes = vault.serialize();

        let decoded = Vault::deserialize(&bytes).unwrap();

        assert_eq!(decoded.entries().len(), 2);
    }
    #[test]
    fn vault_update_entry() {
        let mut vault = Vault::new();

        let mut entry = Entry::new();

        entry.add_record(TlvRecord::new(RecordType::Title, b"Old title".to_vec()));

        let uuid = entry.uuid().unwrap();

        vault.add_entry(entry.clone());

        let mut updated_entry = entry.clone();

        updated_entry.add_record(TlvRecord::new(RecordType::Title, b"New title".to_vec()));

        vault.update_entry(uuid, updated_entry).unwrap();

        let updated = vault.find_entry(uuid).unwrap().unwrap();

        assert_eq!(updated.uuid().unwrap(), uuid);
        assert_eq!(updated.records()[2].value, b"New title");
    }
    #[test]
    fn vault_delete_entry() {
        let mut vault = Vault::new();

        let mut entry1 = Entry::new();
        entry1.add_record(TlvRecord::new(RecordType::Title, b"GitHub".to_vec()));

        let uuid1 = entry1.uuid().unwrap();

        let mut entry2 = Entry::new();
        entry2.add_record(TlvRecord::new(RecordType::Title, b"Bank".to_vec()));

        let uuid2 = entry2.uuid().unwrap();

        vault.add_entry(entry1);
        vault.add_entry(entry2);

        vault.delete_entry(uuid1).unwrap();

        assert_eq!(vault.entries().len(), 1);

        let remaining = vault.find_entry(uuid2).unwrap();

        assert!(remaining.is_some());
    }
    #[test]
    fn vault_rejects_unknown_uuid() {
        let mut vault = Vault::new();

        let unknown_uuid = uuid::Uuid::new_v4();

        let entry = Entry::new();

        assert!(vault.update_entry(unknown_uuid, entry).is_err());
        assert!(vault.delete_entry(unknown_uuid).is_err());
    }

    #[test]
    fn find_entry_by_uuid() {
        let mut vault = Vault::new();

        let mut entry = Entry::new();
        entry.add_record(TlvRecord::new(RecordType::Title, b"GitHub".to_vec()));

        let uuid = entry.uuid().unwrap();

        vault.add_entry(entry);

        let found = vault.find_entry(uuid).unwrap();

        assert!(found.is_some());
        assert_eq!(found.unwrap().records()[1].value, b"GitHub");
    }
    #[test]
    fn entry_from_fields() {
        let entry = Entry::from_fields(
            "GitHub",
            "user123",
            "secret",
            "https://github.com",
            "My GitHub account",
        );

        assert_eq!(entry.records().len(), 6);

        assert_eq!(entry.records()[0].record_type, RecordType::Uuid);

        assert_eq!(entry.records()[1].record_type, RecordType::Title);
        assert_eq!(entry.records()[1].value, b"GitHub");

        assert_eq!(entry.records()[2].record_type, RecordType::Username);
        assert_eq!(entry.records()[2].value, b"user123");

        assert_eq!(entry.records()[3].record_type, RecordType::Password);
        assert_eq!(entry.records()[3].value, b"secret");

        assert_eq!(entry.records()[4].record_type, RecordType::Url);
        assert_eq!(entry.records()[4].value, b"https://github.com");

        assert_eq!(entry.records()[5].record_type, RecordType::Notes);
        assert_eq!(entry.records()[5].value, b"My GitHub account");
    }
}
