use argon2::{
	password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
	Argon2,
};
use rand_core::OsRng;

use crate::error::{AppError, AppResult};

pub fn hash_password(plain: &str) -> AppResult<String> {
	if plain.len() < 4 {
		return Err(AppError::Security(
			"Le mot de passe doit contenir au moins 4 caractères.".into(),
		));
	}
	let salt = SaltString::generate(&mut OsRng);
	let argon2 = Argon2::default();
	argon2
		.hash_password(plain.as_bytes(), &salt)
		.map(|h| h.to_string())
		.map_err(|e| AppError::Security(format!("Erreur de hachage : {e}")))
}

pub fn verify_password(plain: &str, hash: &str) -> AppResult<bool> {
	let parsed = PasswordHash::new(hash).map_err(|e| AppError::Security(format!("Hash invalide : {e}")))?;
	let argon2 = Argon2::default();
	Ok(argon2
		.verify_password(plain.as_bytes(), &parsed)
		.is_ok())
}
