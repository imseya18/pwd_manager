#[cfg(test)]
mod tests;
mod entities;
mod encryption;

use rusqlite::{Result};
use crate::entities::*;
use crate::encryption::*;

fn main() -> Result<()> {
  //let db = init_database()?;
  let database = Database::init("rt\\new.db".to_string())?;
  let new_profil = MasterProfil::new(
      "35".to_string(),
      "sdihfoisdhgoishdogisdhgosidhgosdhohsdhgohdh".to_string(),
      "c'est pas hash".to_string(),
  );
  println!("{:?}", database);
  new_profil.insert(&database.db)?;
  Ok(())
}
