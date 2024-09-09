pub mod database;
pub mod crypto;

pub use database::Database;
pub use crypto::Crypto;
pub use crate::error::MyError;

pub type Result<T> = core::result::Result<T, MyError>;
