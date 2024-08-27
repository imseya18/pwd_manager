#[cfg(test)]
mod tests;
mod entities;
mod encryption;
mod utils;

use std::path::{PathBuf, Path};
use ring::rand::{self, SecureRandom};

use bcrypt::{hash, verify, DEFAULT_COST, BcryptError};
use crate::entities::*;
use crate::encryption::*;
use crate::crypto::*;

fn main() -> Result<()> {

	let database = Database::init(r"stored/data.db")?;
	let db = &database.db;
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
