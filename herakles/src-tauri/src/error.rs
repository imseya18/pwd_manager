use thiserror::Error;
use serde::Serialize;

#[derive(Error, Debug, Serialize)]
pub enum MyError {
    #[error("Database error: {0}")]
    Database(String),

    #[error("I/O error: {0}")]
    Io(String),

    #[error("Password error: {0}")]
    Password(String),

    #[error("Serde error: {0}")]
    Serde(String),

    #[error("UTF-8 error: {0}")]
    Utf8(String),

    #[error("BlockMode error: {0}")]
    Blockmode(String),

    #[error("AES256CBC error: {0}")]
    Aes(String),

    #[error("R2D2 error: {0}")]
    R2d2(String),

    #[error("ChaChaPoly error: {0}")]
    Chachapoly(String),

    #[error("Unknown error: {0}")]
    Unknown(String),
}

impl From<rusqlite::Error> for MyError {
    fn from(error: rusqlite::Error) -> Self {
        MyError::Database(error.to_string())
    }
}

impl From<std::io::Error> for MyError {
    fn from(error: std::io::Error) -> Self {
        MyError::Io(error.to_string())
    }
}

impl From<bcrypt::BcryptError> for MyError {
    fn from(error: bcrypt::BcryptError) -> Self {
        MyError::Password(error.to_string())
    }
}

impl From<serde_json::Error> for MyError {
    fn from(error: serde_json::Error) -> Self {
        MyError::Serde(error.to_string())
    }
}

impl From<std::string::FromUtf8Error> for MyError {
    fn from(error: std::string::FromUtf8Error) -> Self {
        MyError::Utf8(error.to_string())
    }
}

impl From<block_modes::BlockModeError> for MyError {
    fn from(error: block_modes::BlockModeError) -> Self {
        MyError::Blockmode(error.to_string())
    }
}

impl From<block_modes::InvalidKeyIvLength> for MyError {
    fn from(error: block_modes::InvalidKeyIvLength) -> Self {
        MyError::Aes(error.to_string())
    }
}

impl From<r2d2::Error> for MyError {
    fn from(error: r2d2::Error) -> Self {
        MyError::R2d2(error.to_string())
    }
}

impl From<chacha20poly1305::aead::Error> for MyError {
    fn from(error: chacha20poly1305::aead::Error) -> Self {
        MyError::Chachapoly(error.to_string())
    }
}
