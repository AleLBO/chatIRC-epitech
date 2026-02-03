use bcrypt::{hash, verify, DEFAULT_COST};
use crate::errors::{AppError, AppResult};

/// Hash un mot de passe avec bcrypt
pub fn hash_password(password: &str) -> AppResult<String> {
    hash(password, DEFAULT_COST).map_err(|_| AppError::InternalServerError)
}

/// Vérifie un mot de passe contre son hash
pub fn verify_password(password: &str, hash: &str) -> AppResult<bool> {
    verify(password, hash).map_err(|_| AppError::InternalServerError)
}
