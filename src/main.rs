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

fn main() -> Result<(), Box<dyn Error>> {

	let mut database = Database::init(r"stored/data.db")?;

	database.connect()?;
	let test = MasterProfil::new_user(
		"fhdfhdfh",
		"were",
		"c'est pas hash",
		&database.db.ok_or("probleme db")?
	).expect("erreur");

	println!("{:#?}", test);
	Ok(())
}
