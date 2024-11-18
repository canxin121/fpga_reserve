use std::sync::LazyLock;

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher},
    Argon2,
};
use jsonwebtoken::{Algorithm, Validation};

pub mod api;
pub mod db_conn;
pub mod migrations;
pub mod models;

pub static ARGON2: LazyLock<Argon2<'_>> = LazyLock::new(|| Argon2::default());
pub static JWT_VALIDATION: LazyLock<Validation> =
    LazyLock::new(|| Validation::new(Algorithm::HS256));

pub async fn hash_password(password: String) -> Result<String, String> {
    tokio::task::spawn_blocking(move || {
        let salt_string = argon2::password_hash::SaltString::generate(&mut OsRng);
        let password_clone = password.to_string();
        ARGON2
            .hash_password(password_clone.as_bytes(), &salt_string)
            .map_err(|e| format!("Failed to hash password: {:?}", e))
            .map(|password_hash| password_hash.to_string())
    })
    .await
    .map_err(|e| format!("Failed to join blocking task of hash password: {:?}", e))?
}
