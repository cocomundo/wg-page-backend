use anyhow::Result;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};

pub fn create_hash(password: &[u8]) -> Result<String> {
    let salt = SaltString::generate(&mut OsRng);

    // Argon2 with default params (Argon2id v19)
    let argon2 = Argon2::default();

    // Hash password to PHC string ($argon2id$v=19$...)
    let password_hash = argon2.hash_password(&password, &salt)?.to_string();

    Ok(password_hash.to_string())
}

// pub fn verify_password(password: &[u8], hash: &[str]) -> Result<()>{
//     Argon2::default().verify_password(password, PasswordHash::from(hash))
// }
