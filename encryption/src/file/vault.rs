use std::io;

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
    pub fn add_entry(&mut self, entry: Entry) {
        self.entries.push(entry);
    }
    pub fn entries(&self) -> &[Entry] {
        &self.entries
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
}
