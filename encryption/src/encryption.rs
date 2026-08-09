use argon2::{Algorithm, Argon2, Params, Version};
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
