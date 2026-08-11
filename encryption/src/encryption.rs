use aes_gcm::{Aes256Gcm, Key, KeyInit, Nonce, aead::Aead};
use argon2::{Algorithm, Argon2, Params, Version};
use rand::Rng;
pub fn derive_key(password: &[u8], salt: &[u8]) -> [u8; 32] {
    let params = Params::new(
        512 * 1024, // Ram in MIB
        3,          // Iteration
        4,          // Parallel lanes
        Some(32),   // 32-byte output
    )
    .expect("Invalid Argon2 parameters.");

    let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);
    let mut key = [0u8; 32];
    argon2
        .hash_password_into(password, salt, &mut key)
        .expect("Argon2 failed.");

    key
}
pub fn generate_salt() -> [u8; 16] {
    let mut salt = [0u8; 16];
    rand::rng().fill_bytes(&mut salt);
    salt
}

pub fn encrypt(
    key_bytes: &[u8; 32],
    plaintext: &[u8],
) -> Result<(Vec<u8>, [u8; 12]), aes_gcm::Error> {
    let key =
        Key::<Aes256Gcm>::try_from(key_bytes.as_slice()).expect("key must be exactly 32 bytes");
    let cipher = Aes256Gcm::new(&key);

    let mut nonce_bytes = [0u8; 12];
    rand::rng().fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::try_from(nonce_bytes.as_slice()).expect("nonce must be exactly 12 bytes");

    let ciphertext = cipher.encrypt(&nonce, plaintext)?;
    Ok((ciphertext, nonce_bytes))
}

pub fn decrypt(
    key_bytes: &[u8; 32],
    nonce_bytes: &[u8; 12],
    ciphertext: &[u8],
) -> Result<Vec<u8>, aes_gcm::Error> {
    let key =
        Key::<Aes256Gcm>::try_from(key_bytes.as_slice()).expect("key must be exactly 32 bytes");
    let cipher = Aes256Gcm::new(&key);
    let nonce = Nonce::try_from(nonce_bytes.as_slice()).expect("nonce must be exactly 12 bytes");

    cipher.decrypt(&nonce, ciphertext)
}

pub struct VaultFile {
    pub salt: [u8; 16],
    pub nonce: [u8; 12],
    pub cipher_text: Vec<u8>,
}

impl VaultFile {
    pub fn serialize(&self) -> Vec<u8> {
        let mut output = Vec::new();
        // Magic word
        output.extend_from_slice(b"KVLT");
        output.push(1);
        output.extend_from_slice(&self.salt);
        output.extend_from_slice(&self.nonce);
        output.extend_from_slice(&self.cipher_text);

        output
    }
    pub fn deserialize(data: &[u8]) -> Result<Self, String> {
        //NOTE: 4 bytes magic word
        // 1 byte version
        // 16 bytes salt
        // 12 bytes nonce
        // at least 16 bytes GCM tags
        // => 4+1+16+12+16=49

        if data.len() < 49 {
            return Err("vault file is too small".to_string());
        }
        if &data[0..4] != b"KVLT" {
            return Err("invalid vault file".to_string());
        }
        let version = data[4];
        if version != 1 {
            return Err("unsupported vault version".to_string());
        }
        let salt: [u8; 16] = data[5..21].try_into().unwrap();
        let nonce: [u8; 12] = data[21..33].try_into().unwrap();
        let cipher = data[33..].to_vec();

        Ok(Self {
            salt,
            nonce,
            cipher_text: cipher,
        })
    }
}
