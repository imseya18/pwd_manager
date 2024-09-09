#[cfg(test)]
mod tests;
mod entities;
mod encryption;
mod utils;
mod error;
use std::path::{PathBuf, Path};
use ring::rand::{self, SecureRandom};

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = core::result::Result<T, Error>;

use bcrypt::{hash, verify, DEFAULT_COST, BcryptError};
use crate::entities::*;
use crate::encryption::*;
use crate::crypto::*;
use error::MyError;

fn main() -> Result<()> {

	let database = Database::init(r"stored/data.db")?;
	let db = &database.db;
  let new_profil = MasterProfil::get_valide_existing_user("JGLP", "oui c'est ouioui", db)?;
  let mut account = Account::new(6, "first account", "JGLPLOL", "skldngklsndgnkl", None, Some("zob".to_string()), None);
  account.store_in_db(&new_profil.derivated_key.ok_or("err")?, 6, db)?;
  Ok(())
}

// fn main() -> Result<()> {

// 	let mut key = [0u8; 32];
// 	let salt = Crypto::generate_rnd_salt();
// 	let password = "Mot de passe de qualité supérieur";

// 	key = Crypto::create_key_from_password(password, &salt);

// 	let name_man = "Ceci est une donné sensible";

// 	let chiffred_data = Crypto::encrypt(name_man, &key);

// 	println!("{:?} => {}", chiffred_data, Crypto::decrypt(&chiffred_data, &key));

// 	//println!("passwords.push((\"{}\", {:?}, {:?}));", password, salt, key);

// 	let mut database = Database::init(r"stored/test/data.db")?;

// 	let new_profil = MasterProfil::new(
// 		"35".to_string(),
// 		"sdihfoisdhgoishdogisdhgosidhgosdhohsdhgohdh".to_string(),
// 		"c'est pas hash".to_string(),
// 		 "ceci est le salt".to_string()
// 	);
// 	let new_vault = Vault::new("kdsjbfjdb".to_string(), 54, "THIS I MY FIRST VAULT".to_string());
// 	let new_account  = Account::new(64, "uidtest".to_string(), "account 1".to_string(), "JGLP".to_string(), "password".to_string(), None, None, None);
// 	//new_profil.insert(&database.db)?;
// 	//new_vault.insert(&database.db)?;
// 	//new_account.insert(&database.db)?;
//     println!("{:?}", database);
//     //new_profil.insert(&database.db)?;

// 	//println!("{}", database.encrypt(&key)?); //SI DECRYPTEDg

//     Ok(())
// }
