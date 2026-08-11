//NOTE: FILE FORMAT CRATE

const MAGIC: [u8; 4] = *b"KDPR";
const VERSION_MAJOR: u16 = 1;
const VERSION_MINOR: u16 = 0;
const FLAG: u16 = 0;

#[derive(Debug)]
struct FileHeader {
    magic: [u8; 4],
    version_major: u16,
    version_minor: u16,
    flags: u16,
    argon_memory: u32,
    argon_iterations: u32,
    argon_parallelism: u32,
    salt: [u8; 32],
    header_nonce: [u8; 12],
    data_nonce: [u8; 12],
}
// Encoding:
// Binary
// All integers are little-endian
// Offset     Size     Field
// --------------------------------
// 0          4        magic
// 4          2        version_major
// 6          2        version_minor
// 8          2        flags
// 10         4        argon_memory
// 14         4        argon_iterations
// 18         4        argon_parallelism
// 22         32       salt
// 54         12       header_nonce
// 66         12       data_nonce
// --------------------------------
// 78 bytes total

impl FileHeader {
    pub const SIZE: usize = 78;
    pub fn new() -> Self {
        Self {
            magic: MAGIC,
            version_major: VERSION_MAJOR,
            version_minor: VERSION_MINOR,
            flags: FLAG,
            argon_memory: 262144, // 256 * 1024
            argon_iterations: 3,
            argon_parallelism: 2,
            salt: [0u8; 32],
            header_nonce: [0u8; 12],
            data_nonce: [0u8; 12],
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

        buffer[offset..offset + 12].copy_from_slice(&self.header_nonce);
        offset += 12;

        buffer[offset..offset + 12].copy_from_slice(&self.data_nonce);

        buffer
    }
}
