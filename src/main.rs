#[cfg(test)]
mod tests;
mod entities;
mod encryption;

use std::error::Error;
use rusqlite::{Result};
use std::path::{PathBuf, Path};

use crate::entities::*;
use crate::encryption::*;

fn main() -> Result<()> {
  //let db = init_database()?;
  let database = Database::init("new.db".to_string())?;
  let new_profil = MasterProfil::new(
      "35".to_string(),
      "sdihfoisdhgoishdogisdhgosidhgosdhohsdhgohdh".to_string(),
      "c'est pas hash".to_string(),
  );
  let new_vault = Vault::new("kdsjbfjdb".to_string(), 54, "THIS I MY FIRST VAULT".to_string());
  let new_account  = Account::new(64, "uidtest".to_string(), "account 1".to_string(), "JGLP".to_string(), "password".to_string(), None, None, None);
  new_profil.insert(&database.db)?;
  new_vault.insert(&database.db)?;
  new_account.insert(&database.db)?;
  Ok(())
}
