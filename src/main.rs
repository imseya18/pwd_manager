#[cfg(test)]
mod tests;
mod entities;
mod encryption;

use std::path::{PathBuf, Path};

use bcrypt::{hash, verify, DEFAULT_COST, BcryptError};
use crate::entities::*;
use crate::encryption::*;
use crate::crypto::*;

fn main() -> Result<()> {

	let mut database = Database::init(r"stored/data.db")?;

	database.connect()?;
	let db = &database.db.ok_or("probleme db")?;
  let new_profil = MasterProfil::get_by_name("JGLP2", db)?;
  println!("{:#?}", new_profil);
  // let new_vault = Vault::create_store_in_db("sfdgsdg", new_profil.db_id.ok_or("user_id is None")?, "Vault Test", db)?;
  // let new_vault = Vault::create_store_in_db("sdfsdf", new_profil.db_id.ok_or("user_id is None")?, "Vault Test 2", db)?;
  // let new_vault = Vault::create_store_in_db("fdsgsd", new_profil.db_id.ok_or("user_id is None")?, "Vault Test 3", db)?;
  //Vault::get_by_user_id(1, db)?;
  Ok(())
}
