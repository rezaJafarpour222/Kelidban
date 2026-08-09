fn main() {
    let salt = encryption::encryption::generate_salt();

    println!("salt: {:02x?}", salt);
}
