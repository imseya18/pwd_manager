#[cfg(test)]
mod tests;
mod entities;
mod encryption;

use std::error::Error;
use rusqlite::Result;
use std::path::{PathBuf, Path};

use bcrypt::{hash, verify, DEFAULT_COST, BcryptError};
use crate::entities::*;
use crate::encryption::*;
use crate::crypto::*;

fn hash_password(password: String) -> Result<String, BcryptError> {
  hash(password.clone(), DEFAULT_COST)
}


fn main() -> Result<(), Box<dyn Error>> {

	let key = [0u8; 32];

	let hash = hash_password("password".to_string())?;

	let mut database = Database::init(r"stored/data.db")?;

	database.connect()?;
	let test = MasterProfil::new(
		"fhdfhdfh".to_string(),
		"were".to_string(),
		"c'est pas hash".to_string(),
	)
	.hash_password()?
	.self_insert(database.db.as_ref().ok_or("Database not connected")?)?;
	println!("{:#?}", test);
	Ok(())
}
