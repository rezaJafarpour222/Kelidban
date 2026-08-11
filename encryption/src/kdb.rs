//NOTE: FILE FORMAT CRATE

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
