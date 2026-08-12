use std::io;

use crate::{
    encryption::{derive_key, encrypt},
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
