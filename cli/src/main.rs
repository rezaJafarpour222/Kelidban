use std::fs;

use encryption::{
    database::{Entry, Vault},
    encryption::{VaultFile, derive_key, encrypt, generate_salt},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db = encryption::database::Db::new("vault.db")?;
    db.initialize();

    let e = Entry {
        id: None,
        access_key: String::from("ansible"),
        access_token: String::from("TOPSCERETpassWord23123214"),
        token_type: String::from("SSH"),
        desc: None,
        created_at: 12,
        updated_at: 12,
        vault_id: 1,
    };
    let v = Vault {
        id: None,
        name: String::from("Personal"),
        desc: Some(String::from("My Personal Account")),
        created_at: 12,
        updated_at: 12,
    };
    db.add_vault(&v)?;
    db.add_entry(&e)?;
    let password = b"my password";

    // Generate salt and derive the AES-256 key.
    let salt = generate_salt();
    let key = derive_key(password, &salt);
    let salt = generate_salt();
    let key = derive_key(password, &salt);

    let database = std::fs::read("vault.db")?;

    let (ciphertext, nonce) = encrypt(&key, &database)?;

    let vault = VaultFile {
        salt: salt,
        nonce: nonce,
        cipherText: ciphertext,
    };

    let vault_bytes = vault.serialize();

    std::fs::write("vault.enc", vault_bytes)?;

    let file_bytes = std::fs::read("vault.enc")?;

    let restored_vault = VaultFile::deserialize(&file_bytes)?;

    println!("Salt:       {:02x?}", restored_vault.salt);
    println!("Nonce:      {:02x?}", restored_vault.nonce);
    println!("Ciphertext: {} bytes", restored_vault.cipherText.len());
    Ok(())
}
