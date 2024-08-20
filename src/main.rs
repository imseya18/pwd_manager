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
	
	let key = [0u8; 32];

	let mut database = Database::init(r"stored\data.db")?;

	if !database.is_encrypted()? {
		println!("DECRYPTED");
		database.connect()?;
	} else {
		println!("ENCRYPTED");
		database.decrypt(&key)?;
	}

    let new_profil = MasterProfil::new(
            "35".to_string(),
            "sdihfoisdhgoishdogisdhgosidhgosdhohsdhgohdh".to_string(),
            "c'est pas hash".to_string(),
    );
    println!("{:?}", database);
    //new_profil.insert(&database.db)?;

	//println!("{}", database.encrypt(&key)?); //SI DECRYPTEDg

    Ok(())
}
