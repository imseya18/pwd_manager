use crate::{entities::traits::Insertable, Crypto};
use crate::utils::convert_uid_from_db;
use crate::anyhow;
use rusqlite::{Connection, params, Error as RusqliteError};
use bcrypt::{hash, verify, DEFAULT_COST, BcryptError};
use uuid::Uuid;
use super::{Error, Result};

#[derive(Debug)]
pub enum MasterProfileError {
  CrypteError(BcryptError),
  Other(String),
}

impl From<BcryptError> for MasterProfileError {
  fn from(err: BcryptError) -> Self{
    MasterProfileError::CrypteError(err)
  }
}

// impl From<rusqlite::Error> for MasterProfileError {
//   fn from(err: rusqlite::Error) -> Self {
//       MasterProfileError::RuSqliteError(err)
//   }
// }

#[derive(Debug)]
pub struct MasterProfil {
  pub db_id: Option<i64>,
  pub uid: Uuid,
  pub name: String,
  pub master_password: String,
  pub salt: [u8; 16],
  pub derivated_key: Option<[u8; 32]>
}



impl MasterProfil {

  pub fn create_store_in_db(name: impl Into<String>, master_password: impl Into<String>, db: &Connection) -> Result<Self> {
    let mut new_profil = Self::new(name, master_password)?;
    new_profil.hash_password()?;
    new_profil.insert(db)?;
    Ok(new_profil)
  }

  pub fn new(name: impl Into<String>, master_password: impl Into<String>) -> Result<Self>{
      Ok(MasterProfil {
        db_id: None,
        uid: Uuid::new_v4(),
        name: name.into(),
        master_password: master_password.into(),
        salt: Crypto::generate_rnd_salt()?,
        derivated_key: None
      })
  }

  pub fn get_valide_existing_user(name: &str, master_password: &str, db: &Connection) ->Result<Self> {
      let mut user_from_db = Self::get_by_name(name, db)?;
      Self::verify_password(master_password, &user_from_db.master_password)?;
      user_from_db.derivated_key = Some(Crypto::create_key_from_password(master_password, &user_from_db.salt));
      Ok(user_from_db)
  }

  pub fn hash_password(&mut self) -> Result<()>{
      self.master_password = hash(&self.master_password, DEFAULT_COST)?;
      Ok(())
  }

  pub fn self_insert(self, db: &Connection) -> Result<Self> {
      db.execute("INSERT INTO master_profil (uid_profil, name, master_password) VALUES  (?1, ?2, ?3)",
        (&self.uid.to_string(), &self.name, &self.master_password))?;
      Ok(self)
  }

  pub fn get_by_name(name: &str ,db: &Connection) -> Result<Self> {
      let profil = db.query_row("SELECT * FROM master_profil WHERE name = ?1", params![name], |row| {
        let uid = convert_uid_from_db(row.get(1)?)?;
        Ok(MasterProfil {
                        db_id: row.get(0)?,
                        uid,
                        name: row.get(2)?,
                        master_password: row.get(3)?,
                        salt: row.get(4)?,
                        derivated_key: None
                    })
      })?;
      Ok(profil)
  }

  pub fn verify_password(master_password: &str, hash: &str) ->Result<()>{
      if verify(master_password, hash)? {
        Ok(())
      }
      else {
        Err("Invalide Password".into())
      }
    }
}

impl Insertable for MasterProfil {
  fn insert(&self, db: &Connection) -> Result<()> {
      db.execute("INSERT INTO master_profil (uid_profil, name, master_password, salt) VALUES  (?1, ?2, ?3, ?4)",
        (&self.uid.to_string(), &self.name, &self.master_password, &self.salt))?;
      Ok(())
  }

  fn delete(&self, db: &Connection) -> Result<()> {
    let db_id = self.db_id.ok_or("no id_profil value found in struct")?;
    db.execute("DELETE FROM master_profil WHERE id_profil = ?1", params![db_id])?;
    Ok(())
  }
}
