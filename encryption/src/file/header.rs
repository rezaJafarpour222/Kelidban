use std::io;

const MAGIC: [u8; 4] = *b"KDBR";
const VERSION_MAJOR: u16 = 1;
const VERSION_MINOR: u16 = 0;
const FLAGS: u16 = 0;

#[derive(Debug)]
pub struct FileHeader {
    magic: [u8; 4],
    version_major: u16,
    version_minor: u16,
    flags: u16,
    argon_memory: u32,
    argon_iterations: u32,
    argon_parallelism: u32,
    salt: [u8; 32],
    nonce: [u8; 12],
}

/*
 Encoding:
 Binary
 All integers are little-endian
 Offset     Size     Field
 --------------------------------
 0          4        magic
 4          2        version_major
 6          2        version_minor
 8          2        flags
 10         4        argon_memory
 14         4        argon_iterations
 18         4        argon_parallelism
 22         32       salt
 66         12       data_nonce
------------------------------
66 bytes total
*/
impl FileHeader {
    pub const SIZE: usize = 66;
    pub fn new() -> Self {
        Self {
            magic: MAGIC,
            version_major: VERSION_MAJOR,
            version_minor: VERSION_MINOR,
            flags: FLAGS,
            argon_memory: 262144, // 256 * 1024
            argon_iterations: 3,
            argon_parallelism: 2,
            salt: [0u8; 32],
            nonce: [0u8; 12],
        }
    }
    pub fn serialize(&self) -> [u8; Self::SIZE] {
        let mut buffer = [0u8; Self::SIZE];
        let mut offset = 0;
        buffer[offset..offset + 4].copy_from_slice(&self.magic);
        offset += 4;

        buffer[offset..offset + 2].copy_from_slice(&self.version_major.to_le_bytes());
        offset += 2;

        buffer[offset..offset + 2].copy_from_slice(&self.version_minor.to_le_bytes());
        offset += 2;

        buffer[offset..offset + 2].copy_from_slice(&self.flags.to_le_bytes());
        offset += 2;

        buffer[offset..offset + 4].copy_from_slice(&self.argon_memory.to_le_bytes());
        offset += 4;

        buffer[offset..offset + 4].copy_from_slice(&self.argon_iterations.to_le_bytes());
        offset += 4;

        buffer[offset..offset + 4].copy_from_slice(&self.argon_parallelism.to_le_bytes());
        offset += 4;

        buffer[offset..offset + 32].copy_from_slice(&self.salt);
        offset += 32;

        buffer[offset..offset + 12].copy_from_slice(&self.nonce);

        buffer
    }
    pub fn deserialize(bytes: &[u8]) -> io::Result<Self> {
        if bytes.len() != Self::SIZE {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "invalid header size",
            ));
        }

        let mut offset = 0;
        let mut magic = [0u8; 4];

        magic.copy_from_slice(&bytes[offset..offset + 4]);
        offset += 4;

        if magic != MAGIC {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "invalid magic"));
        }

        let version_major = u16::from_le_bytes(bytes[offset..offset + 2].try_into().unwrap());
        offset += 2;

        if version_major != VERSION_MAJOR {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "invalid version",
            ));
        }

        let version_minor = u16::from_le_bytes(bytes[offset..offset + 2].try_into().unwrap());
        offset += 2;

        let flags = u16::from_le_bytes(bytes[offset..offset + 2].try_into().unwrap());
        offset += 2;

        if flags != FLAGS {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "unknown flags"));
        }

        let argon_memory = u32::from_le_bytes(bytes[offset..offset + 4].try_into().unwrap());
        offset += 4;

        let argon_iterations = u32::from_le_bytes(bytes[offset..offset + 4].try_into().unwrap());
        offset += 4;

        let argon_parallelism = u32::from_le_bytes(bytes[offset..offset + 4].try_into().unwrap());
        offset += 4;

        let mut salt = [0u8; 32];
        salt.copy_from_slice(&bytes[offset..offset + 32]);
        offset += 32;

        let mut nonce = [0u8; 12];
        nonce.copy_from_slice(&bytes[offset..offset + 12]);

        Ok(Self {
            magic,
            version_major,
            version_minor,
            flags,
            argon_memory,
            argon_iterations,
            argon_parallelism,
            salt,
            nonce,
        })
    }
    pub fn version(&self) -> String {
        let version = format!("{}:{}", self.version_major, self.version_minor);
        version
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_header_size() {
        let header = FileHeader::new();

        let bytes = header.serialize();

        assert_eq!(bytes.len(), FileHeader::SIZE);
    }

    #[test]
    fn test_header_conversion() {
        let header = FileHeader::new();

        let bytes = header.serialize();

        let decoded = FileHeader::deserialize(&bytes).expect("failed to deserialize header");

        assert_eq!(decoded.magic, header.magic);
        assert_eq!(decoded.version_major, header.version_major);
        assert_eq!(decoded.version_minor, header.version_minor);
        assert_eq!(decoded.flags, header.flags);

        assert_eq!(decoded.argon_memory, header.argon_memory);
        assert_eq!(decoded.argon_iterations, header.argon_iterations);
        assert_eq!(decoded.argon_parallelism, header.argon_parallelism);

        assert_eq!(decoded.salt, header.salt);
        assert_eq!(decoded.nonce, header.nonce);
    }

    #[test]
    fn test_invalid_magic() {
        let mut bytes = FileHeader::new().serialize();

        bytes[0] = b'X';

        let result = FileHeader::deserialize(&bytes);

        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_version() {
        let mut bytes = FileHeader::new().serialize();

        // version_major is at offset 4
        bytes[4] = 2;

        let result = FileHeader::deserialize(&bytes);

        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_flags() {
        let mut bytes = FileHeader::new().serialize();

        // flags are at offset 8
        bytes[8] = 1;

        let result = FileHeader::deserialize(&bytes);

        assert!(result.is_err());
    }

    #[test]
    fn test_version_string() {
        let header = FileHeader::new();

        assert_eq!(header.version(), "1:0");
    }
}
