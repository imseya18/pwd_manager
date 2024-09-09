use thiserror::Error;
use std::fmt;
use std::string::FromUtf8Error;

#[derive(Error, Debug)]
pub enum MyError {
  #[error("Database error: {0}")]
  Databse(#[from] rusqlite::Error),

  #[error("I/O error: {0}")]
  Io(#[from] std::io::Error),

  #[error("Password error: {0}")]
  Password(#[from] bcrypt::BcryptError),

  #[error("Serde error: {0}")]
  Serde(#[from] serde_json::Error),

  #[error("utf8 error: {0}")]
  Utf8(#[from] FromUtf8Error),

  #[error("Unknown error: {0}")]
  Unknown(String),
}
