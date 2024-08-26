pub mod master_profil;
pub mod traits;
pub mod vault;
pub mod account;

use crate::utils::convert_uid_from_db;
pub use account::Account;
pub use master_profil::MasterProfil;
pub use traits::Insertable;
pub use vault::Vault;
pub use master_profil::MasterProfileError;
pub use anyhow::anyhow;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = core::result::Result<T, Error>;
