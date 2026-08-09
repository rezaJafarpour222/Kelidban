fn main() {
    let password = b"my password";
    let salt = b"some random salt";

    let key = encryption::encryption::derive_key(password, salt);

    println!("{:02x?}", key);
}
