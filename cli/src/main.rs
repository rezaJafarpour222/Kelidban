use encryption::kdb::FileHeader;

fn main() {
    // Create a new header
    let header = FileHeader::new();

    // Serialize header into 78 bytes
    let bytes = header.serialize();

    println!("Header size: {} bytes", bytes.len());

    // Print raw bytes
    println!("{:02X?}", bytes);

    // Deserialize bytes back into a header
    let decoded = FileHeader::deserialize(&bytes).expect("failed to deserialize header");

    println!("{:#?}", decoded);
}
