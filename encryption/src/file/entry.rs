use std::io;

use crate::file::record::TlvRecord;

#[derive(Debug, Clone)]
pub struct Entry {
    records: Vec<TlvRecord>,
}

impl Entry {
    pub fn new() -> Self {
        Self {
            records: Vec::new(),
        }
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

        let bytes = entry.serialize();

        let decoded = Entry::deserialize(&bytes).unwrap();
        assert_eq!(decoded.records.len(), 2);
        assert_eq!(decoded.records[0].record_type, RecordType::Title);
        assert_eq!(decoded.records[1].record_type, RecordType::Username);
    }
}
