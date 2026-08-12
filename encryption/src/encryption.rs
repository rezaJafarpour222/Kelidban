use aes_gcm::{Aes256Gcm, Key, KeyInit, Nonce, aead::Aead};
use argon2::{Algorithm, Argon2, Params, Version};
use rand::Rng;
pub fn derive_key(
    password: &[u8],
    salt: &[u8],
    memory: u32,
    iterations: u32,
    parallelism: u32,
) -> [u8; 32] {
    let params =
        Params::new(memory, iterations, parallelism, Some(32)).expect("Invalid Argon2 parameters.");

    let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);

    let mut key = [0u8; 32];

    argon2
        .hash_password_into(password, salt, &mut key)
        .expect("Argon2 failed.");
    key
}
pub fn generate_salt() -> [u8; 32] {
    let mut salt = [0u8; 32];
    rand::rng().fill_bytes(&mut salt);
    salt
}

pub fn encrypt(
    key_bytes: &[u8; 32],
    plaintext: &[u8],
    nonce_bytes: &[u8; 12],
) -> Result<Vec<u8>, aes_gcm::Error> {
    let key =
        Key::<Aes256Gcm>::try_from(key_bytes.as_slice()).expect("Key must be exactly 32 bytes.");

    let cipher = Aes256Gcm::new(&key);

    let nonce = Nonce::try_from(nonce_bytes.as_slice()).expect("Nonce must be exactly 12 bytes.");

    cipher.encrypt(&nonce, plaintext)
}

pub fn decrypt(
    key_bytes: &[u8; 32],
    nonce_bytes: &[u8; 12],
    ciphertext: &[u8],
) -> Result<Vec<u8>, aes_gcm::Error> {
    let key =
        Key::<Aes256Gcm>::try_from(key_bytes.as_slice()).expect("Key must be exactly 32 bytes.");
    let cipher = Aes256Gcm::new(&key);
    let nonce = Nonce::try_from(nonce_bytes.as_slice()).expect("Nonce must be exactly 12 bytes.");

    cipher.decrypt(&nonce, ciphertext)
}
