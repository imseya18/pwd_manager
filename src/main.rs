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
	let test = MasterProfil::get_by_name("were".to_string(), db)?;
	println!("{:#?}", test);
	// let insert_profil = MasterProfil::create_store_new_profil(
 //    "uid3",
 //    "JGLP3", "1234",
 //    db)?;
  let get_profil = MasterProfil::get_valide_existing_user("JGLP5", "1234", db)?;
	Ok(())
}
