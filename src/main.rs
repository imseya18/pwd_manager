#[cfg(test)]
mod tests;
mod entities;
mod encryption;
mod utils;

use std::path::{PathBuf, Path};

use bcrypt::{hash, verify, DEFAULT_COST, BcryptError};
use crate::entities::*;
use crate::encryption::*;
use crate::crypto::*;

fn main() -> Result<()> {

	let mut database = Database::init(r"stored/data.db")?;

	database.connect()?;
	let db = &database.db.ok_or("probleme db")?;
  let new_profil = MasterProfil::get_by_name("JGLP", db)?;
  println!("{:#?}", new_profil);
  let vaults = Vault::get_by_user_id(3, db)?;
  println!("{:?}", vaults.len());
  for vault in vaults.iter(){
    match vault {
      Ok(v) => println!("{:#?}", v),
      Err(e) => println!("{:?}", e)
    }
  }
  Ok(())
}
