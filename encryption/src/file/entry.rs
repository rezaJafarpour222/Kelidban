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
    pub fn uuid(&self) -> io::Result<Uuid> {
        let record = self
            .records
            .iter()
            .find(|record| record.record_type == RecordType::Uuid)
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "entry has no UUID."))?;

        Uuid::from_slice(&record.value)
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "invalid UUID."))
    }
    pub fn from_fields(
        title: &str,
        username: &str,
        password: &str,
        url: &str,
        notes: &str,
    ) -> Self {
        let mut entry = Self::new();

        entry.add_record(TlvRecord::new(RecordType::Title, title.as_bytes().to_vec()));

        entry.add_record(TlvRecord::new(
            RecordType::Username,
            username.as_bytes().to_vec(),
        ));

        entry.add_record(TlvRecord::new(
            RecordType::Password,
            password.as_bytes().to_vec(),
        ));

        entry.add_record(TlvRecord::new(RecordType::Url, url.as_bytes().to_vec()));

        entry.add_record(TlvRecord::new(RecordType::Notes, notes.as_bytes().to_vec()));

        entry
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
}
