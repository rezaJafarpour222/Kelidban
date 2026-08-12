use std::io;
const MAX_RECORD_SIZE: usize = 1024 * 1024 * 2; // 2 MB

#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecordType {
    Title = 0x0001,
    Username = 0x0002,
    Password = 0x0003,
    Url = 0x0004,
    Notes = 0x0005,
    Uuid = 0x0006,
    Created = 0x0007,
    Modified = 0x0008,
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

    pub fn deserialize(bytes: &[u8]) -> io::Result<(Self, usize)> {
        // NOTE: minimum bytes: 2bytes for record + 4bytes length
        if bytes.len() < 6 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "TLV record too small.",
            ));
        }
        let record_type_raw = u16::from_le_bytes(bytes[0..2].try_into().unwrap());
        let record_type = match record_type_raw {
            0x0001 => RecordType::Title,
            0x0002 => RecordType::Username,
            0x0003 => RecordType::Password,
            0x0004 => RecordType::Url,
            0x0005 => RecordType::Notes,
            0x0006 => RecordType::Uuid,
            0x0007 => RecordType::Created,
            0x0008 => RecordType::Modified,
            _ => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Unknown record type.",
                ));
            }
        };
        let length = u32::from_le_bytes(bytes[2..6].try_into().unwrap()) as usize;
        if length > MAX_RECORD_SIZE {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Record is too large.",
            ));
        }
        if bytes.len() < 6 + length {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "truncated TLV record.",
            ));
        }
        // NOTE: if length of value is 50 , and value start from 6 , then i need to read from 6 to 56
        let value = bytes[6..6 + length].to_vec();

        Ok((Self { record_type, value }, 6 + length))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tlv_conversion() {
        let record = TlvRecord::new(RecordType::Password, b"secret".to_vec());

        let bytes = record.serialize();

        let (decoded, _) = TlvRecord::deserialize(&bytes).unwrap();

        assert_eq!(decoded.record_type, RecordType::Password);
        assert_eq!(decoded.value, b"secret");
    }

    #[test]
    fn reject_invalid_length() {
        let bytes = [
            0x04, 0x00, // Password
            0x05, 0x00, 0x00, 0x00, // claims 5 bytes
            b'a', b'b',
        ];

        assert!(TlvRecord::deserialize(&bytes).is_err());
    }
    #[test]
    fn all_record_types_roundtrip() {
        let types = [
            RecordType::Title,
            RecordType::Username,
            RecordType::Password,
            RecordType::Url,
            RecordType::Notes,
            RecordType::Uuid,
            RecordType::Created,
            RecordType::Modified,
        ];

        for record_type in types {
            let record = TlvRecord::new(record_type, vec![1, 2, 3]);

            let bytes = record.serialize();

            let (decoded, _) = TlvRecord::deserialize(&bytes).unwrap();

            assert_eq!(decoded.record_type, record_type);
            assert_eq!(decoded.value, vec![1, 2, 3]);
        }
    }
}
