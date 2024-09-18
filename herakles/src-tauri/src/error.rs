use thiserror::Error;
use std::fmt;
use std::string::FromUtf8Error;
use block_modes::{BlockModeError, InvalidKeyIvLength};
use chacha20poly1305::aead::Error as ChachaPolyError;

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

  #[error("BlockMode error: {0}")]
  Blockmode(#[from] BlockModeError),

  #[error("AES256CBc error: {0}")]
  Aes(#[from] InvalidKeyIvLength),

  #[error("R2D2 error: {0}")]
  R2d2(#[from]r2d2::Error),

  #[error("ChaChaPoly error: {0}")]
  Chachapoly(String),

  #[error("Unknown error: {0}")]
  Unknown(String),
}
