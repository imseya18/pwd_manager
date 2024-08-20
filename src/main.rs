#[cfg(test)]
mod tests;
mod entities;
mod encryption;

use std::error::Error;
use rusqlite::{Result};
use std::path::{PathBuf, Path};

use crate::entities::*;
use crate::encryption::*;

fn main() -> Result<(), Box<dyn Error>> {
	
	let db_path = Database::path(r"stored\data.db")?;
    let database = Database::init(db_path)?;

    let new_profil = MasterProfil::new(
            "35".to_string(),
            "sdihfoisdhgoishdogisdhgosidhgosdhohsdhgohdh".to_string(),
            "c'est pas hash".to_string(),
    );
    println!("{:?}", database);
    //new_profil.insert(&database.db)?;

    let key = [0u8; 32];
	println!("{}", database.encrypt(&key)?);

    Ok(())
}
