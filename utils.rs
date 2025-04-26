use bcrypt::{hash, BcryptResult, DEFAULT_COST};

// Utility function to hash a password using bcrypt.
pub fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {
    hash(password, DEFAULT_COST)
}
