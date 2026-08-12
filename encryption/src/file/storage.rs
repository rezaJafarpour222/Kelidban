use std::{fs, io, path::Path};

use crate::{
    encryption::{decrypt, derive_key, encrypt},
    file::{header::FileHeader, vault::Vault},
};

pub fn encrypt_vault(password: &[u8], header: &FileHeader, vault: &Vault) -> io::Result<Vec<u8>> {
    let key = derive_key(
        password,
        &header.salt,
        header.argon_memory,
        header.argon_iterations,
        header.argon_parallelism,
    );
    let plaintext = vault.serialize();
    let ciphertext = encrypt(&key, &plaintext, &header.nonce)
        .map_err(|_| io::Error::other("failed to encrypt the vault."))?;

    let mut output = Vec::with_capacity(FileHeader::SIZE + ciphertext.len());
    output.extend_from_slice(&header.serialize());
    output.extend_from_slice(&ciphertext);

    Ok(output)
}
pub fn decrypt_vault(password: &[u8], data: &[u8]) -> io::Result<Vault> {
    if data.len() < FileHeader::SIZE {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "vault file is too small.",
        ));
    }
    let header_bytes = &data[..FileHeader::SIZE];
    let ciphertext = &data[FileHeader::SIZE..];

    let header = FileHeader::deserialize(header_bytes)?;
    let key = derive_key(
        password,
        &header.salt,
        header.argon_memory,
        header.argon_iterations,
        header.argon_parallelism,
    );

    let plaintext = crate::encryption::decrypt(&key, &header.nonce, ciphertext)
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "failed to decrypt vault."))?;

    Vault::deserialize(&plaintext)
}
pub fn save_vault<P: AsRef<Path>>(path: P, password: &[u8], vault: &Vault) -> io::Result<()> {
    let header = FileHeader::new();
    let data = encrypt_vault(password, &header, vault)?;

    fs::write(path, data)?;

    Ok(())
}

pub fn load_vault<P: AsRef<Path>>(path: P, password: &[u8]) -> io::Result<Vault> {
    let data = fs::read(path)?;
    decrypt_vault(password, &data)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::file::entry::Entry;
    use crate::file::record::{RecordType, TlvRecord};

    #[test]
    fn vault_encrypt_decrypt_roundtrip() {
        let password = b"test-password";

        let mut vault = Vault::new();

        let mut entry = Entry::new();

        entry.add_record(TlvRecord::new(RecordType::Title, b"GitHub".to_vec()));

        entry.add_record(TlvRecord::new(RecordType::Username, b"user123".to_vec()));

        vault.add_entry(entry);

        let header = FileHeader::new();

        let encrypted = encrypt_vault(password, &header, &vault).unwrap();

        let decrypted = decrypt_vault(password, &encrypted).unwrap();

        assert_eq!(decrypted.entries().len(), 1);

        let entry = &decrypted.entries()[0];

        assert_eq!(entry.records().len(), 2);

        assert_eq!(entry.records()[0].record_type, RecordType::Title);

        assert_eq!(entry.records()[1].record_type, RecordType::Username);
    }
    #[test]
    fn wrong_password_fails() {
        let correct_password = b"correct-password";
        let wrong_password = b"wrong-password";

        let mut vault = Vault::new();

        let mut entry = Entry::new();

        entry.add_record(TlvRecord::new(RecordType::Title, b"GitHub".to_vec()));

        vault.add_entry(entry);

        let header = FileHeader::new();

        let encrypted = encrypt_vault(correct_password, &header, &vault).unwrap();

        let result = decrypt_vault(wrong_password, &encrypted);

        assert!(result.is_err());
    }
    #[test]
    fn tampered_ciphertext_fails() {
        let password = b"correct-password";

        let mut vault = Vault::new();

        let mut entry = Entry::new();
        entry.add_record(TlvRecord::new(RecordType::Title, b"GitHub".to_vec()));

        vault.add_entry(entry);

        let header = FileHeader::new();

        let mut encrypted = encrypt_vault(password, &header, &vault).unwrap();

        // Change one byte in the ciphertext.
        encrypted[FileHeader::SIZE] ^= 1;

        let result = decrypt_vault(password, &encrypted);

        assert!(result.is_err());
    }
    #[test]
    fn save_and_load_vault() {
        let password = b"test-password";
        let path = "test-vault.kdb";

        let mut vault = Vault::new();

        let mut entry = Entry::new();

        entry.add_record(TlvRecord::new(RecordType::Title, b"GitHub".to_vec()));

        vault.add_entry(entry);

        save_vault(path, password, &vault).unwrap();

        let loaded = load_vault(path, password).unwrap();

        assert_eq!(loaded.entries().len(), 1);
        assert_eq!(
            loaded.entries()[0].records()[0].record_type,
            RecordType::Title
        );

        std::fs::remove_file(path).unwrap();
    }
    #[test]
    fn load_with_wrong_password_fails() {
        let correct_password = b"correct-password";
        let wrong_password = b"wrong-password";
        let path = "test-vault.kdb";

        let mut vault = Vault::new();

        let mut entry = Entry::new();

        entry.add_record(TlvRecord::new(RecordType::Title, b"GitHub".to_vec()));

        vault.add_entry(entry);

        save_vault(path, correct_password, &vault).unwrap();

        let result = load_vault(path, wrong_password);

        assert!(result.is_err());

        std::fs::remove_file(path).unwrap();
    }
    #[test]
    fn truncated_vault_file_fails() {
        let password = b"test-password";
        let path = "test-vault.kdb";

        let mut vault = Vault::new();

        let mut entry = Entry::new();

        entry.add_record(TlvRecord::new(RecordType::Title, b"GitHub".to_vec()));

        vault.add_entry(entry);

        save_vault(path, password, &vault).unwrap();

        // Read the complete file.
        let mut data = std::fs::read(path).unwrap();

        // Remove the last byte.
        data.pop();

        // Write the damaged file back.
        std::fs::write(path, data).unwrap();

        let result = load_vault(path, password);

        assert!(result.is_err());

        std::fs::remove_file(path).unwrap();
    }
}
