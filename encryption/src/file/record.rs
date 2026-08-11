use std::io;

#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecordType {
    Entry = 0x0001,
    Title = 0x0002,
    Username = 0x0003,
    Password = 0x0004,
    Url = 0x0005,
    Notes = 0x0006,
}

#[derive(Debug, Clone)]
pub struct TlvRecord {
    pub record_type: RecordType,
    pub value: Vec<u8>,
}
impl TlvRecord {
    pub fn new(record_type: RecordType, value: Vec<u8>) -> Self {
        Self { record_type, value }
    }
    pub fn serialize(&self) -> Vec<u8> {
        let mut buffer = Vec::new();

        buffer.extend_from_slice(&(self.record_type as u16).to_le_bytes());
        buffer.extend_from_slice(&(self.value.len() as u32).to_le_bytes());
        buffer.extend_from_slice(&self.value);

        buffer
    }

    pub fn deserialize(bytes: &[u8]) -> io::Result<Self> {
        // minimum bytes: 2bytes for record + 4bytes length
        if bytes.len() < 6 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "TLV record too small",
            ));
        }
        let record_type_raw = u16::from_le_bytes(bytes[0..2].try_into().unwrap());
        let record_type = match record_type_raw {
            0x0001 => RecordType::Entry,
            0x0002 => RecordType::Title,
            0x0003 => RecordType::Username,
            0x0004 => RecordType::Password,
            0x0005 => RecordType::Url,
            0x0006 => RecordType::Notes,
            _ => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "unknown record type",
                ));
            }
        };
        let length = u32::from_le_bytes(bytes[2..6].try_into().unwrap()) as usize;
        if bytes.len() != 6 + length {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "invalid TLV length",
            ));
        }
        let value = bytes[6..].to_vec();

        Ok(Self { record_type, value })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tlv_conversion() {
        let record = TlvRecord::new(RecordType::Password, b"secret".to_vec());

        let bytes = record.serialize();

        let decoded = TlvRecord::deserialize(&bytes).unwrap();

        assert_eq!(decoded.record_type, RecordType::Password);
        assert_eq!(decoded.value, b"secret");
    }

    #[test]
    fn reject_unknown_type() {
        let bytes = [
            0x00, 0x00, // invalid type
            0x00, 0x00, 0x00, 0x00,
        ];

        assert!(TlvRecord::deserialize(&bytes).is_err());
    }
}
