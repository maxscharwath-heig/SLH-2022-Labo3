use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use rand_core::OsRng;

pub fn verify_hash(password: String, hash: String) -> bool {
    let argon2 = Argon2::default();
    match PasswordHash::new(hash.as_str()) {
        Ok(hash) => argon2.verify_password(password.as_bytes(), &hash).is_ok(),
        _ => false,
    }
}

pub fn hash_password(password: String) -> String {
    let argon2 = Argon2::default();
    let salt = SaltString::generate(&mut OsRng);
    let hash = argon2
        .hash_password(password.as_bytes(), salt.as_ref())
        .unwrap();
    hash.to_string()
}
