use serde::Serialize;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
	#[error("{0}")]
	Message(String),
	#[error("Base de données : {0}")]
	Db(#[from] rusqlite::Error),
	#[error("I/O : {0}")]
	Io(#[from] std::io::Error),
	#[error("{0}")]
	Network(String),
	#[error("PHP : {0}")]
	Php(String),
	#[error("Sécurité : {0}")]
	Security(String),
	#[error("{0}")]
	Json(String),
}

impl From<String> for AppError {
	fn from(value: String) -> Self {
		AppError::Message(value)
	}
}

impl From<serde_json::Error> for AppError {
	fn from(value: serde_json::Error) -> Self {
		AppError::Json(value.to_string())
	}
}

impl Serialize for AppError {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		serializer.serialize_str(&self.to_string())
	}
}

pub type AppResult<T> = Result<T, AppError>;
